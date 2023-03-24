use std::{mem, ptr};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, size_of};

use crate::{BytesType, LenType, MetaKey, read_int_ptr, write_int_ptr};

#[derive(Debug, Clone)]
pub struct BytesMeta {
    offset: i64, //这个值由于要持久化，所以不能使用isize
}

impl BytesMeta {
    pub fn new(offset: isize) -> Self {
        Self {
            offset: offset as i64
        }
    }
    pub fn to_isize(&self) -> isize {
        self.offset as isize
    }
    pub fn set_offset(&mut self, offset: isize) {
        self.offset = offset as i64;
    }
}

impl From<isize> for BytesMeta {
    fn from(value: isize) -> Self {
        BytesMeta::new(value)
    }
}

impl From<usize> for BytesMeta {
    fn from(value: usize) -> Self {
        BytesMeta::new(value as isize)
    }
}

/// 前面存放meta信息，所key的offset(相对于第一个key的位置，这样当增加meta时，原meta中的offset不用修改)，key
/// 这样设计是为了使用二分查找，因为key长度不固定
#[derive(Clone, Debug)]
pub struct VecBytes<T: Metas> {
    pub number_keys: LenType,
    /// 单位是 byte, 所有keys合起来的，包含前后的 BytesType
    pub bytes_data: BytesType,
    /// 为meta数据分配的容量
    pub metas_capacity: u64,

    //在整个数据中的offset
    pub offset: isize,

    _mark: PhantomData<T>,
}

impl<T: Metas> VecBytes<T> {
    pub const offset_data: isize = (size_of::<LenType>() + size_of::<BytesType>()) as isize;
    pub const offset_meta: isize = Self::offset_data + size_of::<u64>() as isize;
    ///一次分配大小，这样不用每增加一个元素就分配一次
    pub const ONE_EXPAND: isize = 64 * (size_of::<BytesMeta>() as isize);

    pub fn new() -> Self {
        // Vec::new()
        // core::marker::PhantomData
        Self {
            number_keys: 0,
            bytes_data: 0,
            metas_capacity: 0,
            offset: 0,
            _mark: PhantomData,
        }
    }

    /// keys已排序
    pub fn add(data: &mut Vec<u8>, offset: isize, keys_: &[&[u8]]) -> Self {
        let mut keys = VecBytes::new();
        keys.number_keys = keys_.len() as LenType;
        keys.offset = offset;
        if keys.number_keys as usize * size_of::<BytesMeta>() >= keys.metas_capacity as usize {
            keys.expand(data);
        }

        for index in 0..keys_.len() {
            keys.insert_with_index(data, keys_[index], Some(index as usize));
        }
        keys
    }

    pub fn read_from(&mut self, data: &[u8], offset: isize) {
        self.offset = offset;
        unsafe {
            self.number_keys = read_int_ptr(data.as_ptr().offset(self.offset));
            self.bytes_data = read_int_ptr(data.as_ptr().offset(self.offset + size_of::<LenType>() as isize));
            self.metas_capacity = read_int_ptr(data.as_ptr().offset(Self::offset_data));
        }
    }

    /// 在index处插入key,
    pub fn insert_with_index(&mut self, data: &mut Vec<u8>, key: &[u8], index: Option<usize>) {
        if self.is_expand() {
            self.expand(data);
        }
        let mut add_bytes = Self::compute_new_bytes(key);
        data.reserve_exact(add_bytes as usize);

        let start = self.offset + Self::offset_data + self.metas_capacity as isize + self.bytes_data as isize;
        unsafe {
            write_int_ptr(data.as_mut_ptr().offset(start), key.len() as BytesType);
            write_int_ptr(data.as_mut_ptr().offset(start + size_of::<BytesType>() as isize + key.len() as isize), key.len() as BytesType);
            ptr::copy(key.as_ptr(), data.as_mut_ptr().offset(start + size_of::<BytesType>() as isize), key.len());
        }

        {
            let mut meta = T::new(data, Self::offset_meta, self.number_keys, (self.metas_capacity as usize / size_of::<MetaKey>()) as i64);
            let index = match index {
                Some(i) => i,
                None => {
                    match meta.search(data.as_slice(), self.offset + Self::offset_meta + self.metas_capacity as isize, key) {
                        Ok(i) => i,
                        Err(i) => i
                    }
                }
            };
            meta.insert(index, BytesMeta::from(start))
        }
        unsafe { data.set_len(data.len() + add_bytes as usize); }
        self.set_number_keys(self.number_keys + 1 as LenType, data);
        self.set_bytes_data(self.bytes_data + add_bytes as BytesType, data);
    }
    /// offset keys中node中的偏移量，
    pub fn inserts(&mut self, data: &mut Vec<u8>, keys: &[&[u8]]) {
        for key in keys {
            self.insert_with_index(data, key, None);
        }
    }


    pub fn set_number_keys(&mut self, number_keys: LenType, data: &mut [u8]) {
        self.number_keys = number_keys;
        unsafe { write_int_ptr(data.as_mut_ptr().offset(self.offset), self.number_keys); }
    }
    pub fn set_bytes_data(&mut self, bytes_number: BytesType, data: &mut [u8]) {
        self.bytes_data = bytes_number;
        unsafe { write_int_ptr(data.as_mut_ptr().offset(self.offset + size_of::<LenType>() as isize), self.bytes_data); }
    }

    pub fn binary_search(&self, data: &[u8], key: &[u8]) -> Result<usize, usize> {
        let mut data = data;
        let mut meta = T::new(data, Self::offset_meta, self.number_keys, (self.metas_capacity as usize / size_of::<MetaKey>()) as i64);
        meta.search(data, self.offset + Self::offset_meta + self.metas_capacity as isize, key)
    }

    fn compute_new_bytes(key: &[u8]) -> BytesType {
        size_of::<BytesType>() as BytesType * 2 + key.len() as BytesType
    }

    /// 是否需要扩展meta的空间
    fn is_expand(&self) -> bool {
        if self.number_keys as usize * size_of::<BytesMeta>() >= self.metas_capacity as usize {
            return true;
        }
        return false;
    }
    fn expand(&mut self, data: &mut Vec<u8>) {
        let expand_size = Self::ONE_EXPAND;
        data.reserve_exact(expand_size as usize);
        unsafe {
            data.set_len(data.len() + expand_size as usize);
        }
        let old_metas_capacity = self.metas_capacity as isize;
        unsafe {
            let p_data = data.as_mut_ptr().offset(Self::offset_meta + old_metas_capacity);
            ptr::copy(p_data, p_data.offset(expand_size as isize), data.len() - expand_size as usize - Self::offset_meta as usize - old_metas_capacity as usize);
            write_int_ptr(data.as_mut_ptr().offset(size_of::<LenType>() as isize), old_metas_capacity as LenType + expand_size as LenType);
        };
        self.metas_capacity = old_metas_capacity as u64 + expand_size as u64;
    }

    fn reduce(&mut self, data: &mut Vec<u8>) {
        let reduce_size = Self::ONE_EXPAND as isize;
        let mut temp_fields = Vec::<u8>::with_capacity(data.len() - self.metas_capacity as usize - Self::offset_meta as usize);

        let mut head_array = unsafe {
            Vec::from_raw_parts(data.as_mut_ptr().offset(Self::offset_meta as isize) as *mut BytesMeta, self.number_keys as usize, self.metas_capacity as usize / mem::size_of::<BytesMeta>())
        };
        let mut offset = 0;
        let p_data = unsafe { data.as_ptr().offset(Self::offset_meta + self.metas_capacity as isize) };
        for field_meta in &mut head_array {
            let start = field_meta.offset as isize;
            let field_size = unsafe { read_int_ptr::<BytesType>(p_data.offset(start)) };
            unsafe {
                ptr::copy_nonoverlapping(p_data.offset(start), temp_fields.as_mut_ptr().offset(offset), field_size as usize + size_of::<BytesType>());
            }
            field_meta.set_offset(offset);
            offset += field_size as isize + size_of::<BytesType>() as isize;
            unsafe { temp_fields.set_len(offset as usize); }
        }
        let _ = ManuallyDrop::new(head_array);
        self.metas_capacity -= reduce_size as u64;
        unsafe {
            temp_fields.set_len(offset as usize);
            write_int_ptr(data.as_mut_ptr().offset(size_of::<LenType>() as isize), self.metas_capacity as LenType);
            ptr::copy_nonoverlapping(temp_fields.as_ptr(), data.as_mut_ptr().offset(Self::offset_meta + self.metas_capacity as isize), temp_fields.len());
            data.set_len((Self::offset_meta as usize + self.metas_capacity as usize + temp_fields.len()) as usize);
        }
    }
}

pub trait Metas {
    fn new(data: &[u8], offset: isize, len: LenType, cap: i64) -> Self;
    fn insert(&mut self, index: usize, key_meta: BytesMeta);
    fn search(&self, data: &[u8], offset: isize, key: &[u8]) -> Result<usize, usize>;
}

#[derive(Debug, Clone)]
pub struct KeyMetas {
    //这个数据是一个引用
    data: Vec<BytesMeta>,
}

impl Drop for KeyMetas {
    fn drop(&mut self) {
        let data = mem::replace(&mut self.data, vec![]);
        mem::forget(data);
    }
}

impl Metas for KeyMetas {
    fn new(data: &[u8], offset: isize, len: LenType, cap: i64) -> Self {
        unsafe {
            Self {
                data: Vec::from_raw_parts(data.as_ptr().offset(offset) as *mut _, len as usize, cap as usize),
            }
        }
    }
    fn insert(&mut self, index: usize, key_meta: BytesMeta) {
        self.data.insert(index, key_meta);
    }

    fn search(&self, data: &[u8], offset: isize, key: &[u8]) -> Result<usize, usize> {
        unsafe {
            self.data.binary_search_by(|a| {
                let mut start = offset + a.to_isize();
                let bytes_ = read_int_ptr::<BytesType>(data.as_ptr().offset(start));
                start += size_of::<BytesType>() as isize;
                let o_key = &data[start as usize..start as usize + bytes_ as usize];
                return key.cmp(o_key);
            })
        }
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_vec() {}
}

