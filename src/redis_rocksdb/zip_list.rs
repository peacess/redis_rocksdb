use core::ptr;

use ckb_rocksdb::{Transaction, TransactionDB};
use ckb_rocksdb::prelude::Get;

use crate::{EndianScalar, Error, LenType, read_int, read_len_type, SIZE_LEN_TYPE, write_int};

///
/// ```rust
/// use redis_rocksdb::{LenType, MetaKey};
///
/// struct ZipList{
///     len: LenType,
///     values: [u8],
/// }
/// ```
pub(crate) struct ZipList(Vec<u8>);

impl From<Vec<u8>> for ZipList {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes;
        if bytes.len() < ZipList::LEN_INIT {
            bytes.resize(ZipList::LEN_INIT, 0);
        }
        ZipList(bytes)
    }
}

impl AsRef<[u8]> for ZipList {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ZipList {
    const LEN_INIT: usize = 32;
    const OFFSET_VALUE: usize = SIZE_LEN_TYPE;
    pub fn new() -> Self {
        ZipList(Vec::from([0; ZipList::LEN_INIT]))
    }

    #[inline]
    fn write_value(value: &[u8], p: *mut u8) {
        let x_le = (value.len() as LenType).to_little_endian();
        unsafe {
            ptr::copy_nonoverlapping(
                &x_le as *const LenType as *const u8,
                p,
                SIZE_LEN_TYPE,
            );
            ptr::copy_nonoverlapping(
                value.as_ptr(),
                p.offset(SIZE_LEN_TYPE as isize),
                value.len(),
            );
        }
    }

    pub(crate) fn get(tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<ZipList>, Error> {
        let v = tr.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                Ok(Some(ZipList::from(v.to_vec())))
            }
        }
    }

    pub fn len(&self) -> LenType {
        read_int(self.0.as_slice())
    }

    pub fn set_len(&mut self, len: LenType) {
        write_int(self.0.as_mut_slice(), len)
    }

    pub fn pop_left(&mut self) -> Vec<u8> {
        let len = self.len() - 1;
        self.set_len(len);

        let offset = ZipList::OFFSET_VALUE;
        let value_len = read_len_type(&self.0[offset..]) as usize;
        let mut pop_value = Vec::with_capacity(value_len);
        unsafe {
            pop_value.set_len(value_len);
            ptr::copy_nonoverlapping(
                self.0[offset + SIZE_LEN_TYPE..].as_ptr(),
                pop_value.as_mut_ptr(),
                pop_value.len(),
            );
        }

        let value_all_len = SIZE_LEN_TYPE + value_len;
        unsafe {
            let p = self.0[offset..].as_mut_ptr();
            ptr::copy(p.offset(value_all_len as isize), p, value_all_len);
        }
        self.0.truncate(self.0.len() - value_all_len);

        pop_value
    }

    pub fn push_left(&mut self, value: &[u8]) {
        self.insert_left(0, value);
    }

    pub fn pop_right(&mut self) -> Vec<u8> {
        let len = self.len() - 1;
        self.set_len(len);

        let mut offset = 0;
        for _ in 0..len {
            let value_len = read_len_type(&self.0[offset..]) as usize;
            offset += value_len + SIZE_LEN_TYPE;
        }

        let value_len = read_len_type(&self.0[offset..]) as usize;
        let mut pop_value = Vec::with_capacity(value_len);
        unsafe {
            pop_value.set_len(value_len);
            ptr::copy_nonoverlapping(
                self.0[offset..].as_ptr(),
                pop_value.as_mut_ptr(),
                pop_value.len(),
            );
        }

        self.0.truncate(offset);
        pop_value
    }

    pub fn push_right(&mut self, value: &[u8]) {
        let s = self.len() + 1;
        self.set_len(s);

        let old_len = self.0.len();
        let add_len = SIZE_LEN_TYPE + value.len();

        // fn resize will set the default value, so replace with  reserve and set_len
        self.0.reserve(add_len);
        unsafe { self.0.set_len(old_len + add_len); }
        unsafe { ZipList::write_value(value, self.0.as_mut_ptr().offset(old_len as isize)); }
    }

    pub fn insert_left(&mut self, index: i32, value: &[u8]) {
        // todo if the index > value_len
        let s = self.len() + 1;
        self.set_len(s);

        let old_len = self.0.len();
        let add_len = SIZE_LEN_TYPE + value.len();
        self.0.reserve(add_len);
        unsafe { self.0.set_len(old_len + add_len); }
        unsafe {
            let offset = {
                let mut offset = 0;
                if index < self.len() as i32 - 1 {
                    for _ in 1..index + 1 {
                        let value_len = read_len_type(&self.0[offset..]) as usize;
                        offset += value_len + SIZE_LEN_TYPE;
                    }
                    let p = self.0.as_mut_ptr().offset(offset as isize);
                    // Shift everything over to make space. (Duplicating the
                    // `index`th element into two consecutive places.)
                    ptr::copy(p, p.offset(add_len as isize), old_len - add_len);
                }// else时，相当于 push_right
                offset
            };

            ZipList::write_value(value, self.0.as_mut_ptr().offset(offset as isize));
        }
    }

    pub fn insert_right(&mut self, index: i32, value: &[u8]) {
        self.insert_left(index + 1, value)
    }

    /// 没有找到pivot 返回None
    /// 找到并成功插入，返回插入后的 offset
    pub fn insert_value_left(&mut self, pivot: &[u8], value: &[u8]) -> Option<i32> {
        let old_bytes_len = self.0.len();
        let mut offset = 0;
        //因为要表示没有找到，所以使用 i32类型
        let mut find_len = -1;
        for _ in 0..old_bytes_len {
            let value_len = read_len_type(&self.0[offset..]) as usize;
            let value_now = &self.0[offset + SIZE_LEN_TYPE..offset + SIZE_LEN_TYPE + value_len];
            if value_now.eq(pivot) {
                find_len = value_len as i32;
                break;
            }
            offset += value_len + SIZE_LEN_TYPE;
        }

        // 找到数据
        if find_len != -1 {
            unsafe {
                let add_len = SIZE_LEN_TYPE + value.len();
                self.0.reserve(add_len);
                self.0.set_len(self.0.len() + add_len);

                let p = self.0.as_mut_ptr().offset(offset as isize);
                ptr::copy(p, p.offset(add_len as isize), old_bytes_len - offset);
            }
            unsafe { ZipList::write_value(value, self.0.as_mut_ptr().offset(offset as isize)) }
            self.set_len(self.len() + 1);
            return Some(offset as i32)
        }else{
            None
        }
    }

    /// 没有找到pivot 返回None
    /// 找到并成功插入，返回插入后的 offset
    pub fn insert_value_right(&mut self, pivot: &[u8], value: &[u8])-> Option<i32> {
        let old_bytes_len = self.0.len();
        let mut offset = 0;
        //因为要表示没有找到，所以使用 i32类型
        let mut find_len = -1;
        for _ in 0..old_bytes_len {
            let value_len = read_len_type(&self.0[offset..]) as usize;
            let value_now = &self.0[offset + SIZE_LEN_TYPE..offset + SIZE_LEN_TYPE + value_len];
            if value_now.eq(pivot) {
                find_len = value_len as i32;
                break;
            }
            offset += value_len + SIZE_LEN_TYPE;
        }

        // 找到数据
        if find_len != -1 {
            let add_len = SIZE_LEN_TYPE + value.len();
            offset += add_len;
            unsafe {
                self.0.reserve(add_len);
                self.0.set_len(self.0.len() + add_len);

                let p = self.0.as_mut_ptr().offset(offset as isize);
                ptr::copy(p, p.offset(add_len as isize), old_bytes_len - offset);
            }
            unsafe { ZipList::write_value(value, self.0.as_mut_ptr().offset(offset as isize)) }
            self.set_len(self.len() + 1);
            return Some(offset as i32)
        }else{
            None
        }
    }

    pub fn set(&mut self, index: i32, value: &[u8]) {
        // todo if the index > value_len
        unsafe {
            let offset = {
                let mut offset = 0;
                let mut old_value_len: usize = 0;
                for _ in 0..index + 1 {
                    old_value_len = read_len_type(&self.0[offset..]) as usize;
                    offset += old_value_len + SIZE_LEN_TYPE;
                }
                offset -= old_value_len + SIZE_LEN_TYPE;

                let p = self.0.as_mut_ptr().offset(offset as isize);
                //这里一定要使用isize,因为可能为负数
                let diff: isize = value.len() as isize - (old_value_len as isize);
                if diff == 0 {
                    //这种情况下 不需要移动任何数据，因为大小是一样的
                } else if diff > 0 {
                    self.0.reserve(diff as usize);
                    self.0.set_len(self.0.len() + diff as usize);
                    ptr::copy(p, p.offset(diff as isize), self.0.len() - offset - diff as usize);
                } else if diff < 0 {
                    ptr::copy(p, p.offset(diff), self.0.len() - offset);
                    self.0.truncate(self.0.len() - diff as usize);
                }
                offset as isize
            };

            ZipList::write_value(value, self.0.as_mut_ptr().offset(offset));
        }
    }

    pub fn index(&self, index: i32) -> Option<&[u8]> {
        if index >= self.len() as i32 {
            return None;
        }
        let mut offset = ZipList::OFFSET_VALUE;
        for _ in 0..index {
            let size_value = read_len_type(&self.0[offset..]);
            offset += SIZE_LEN_TYPE + size_value as usize;
        }
        let size_value = read_len_type(&self.0[offset..]);
        Some(&self.0[offset + SIZE_LEN_TYPE..offset + SIZE_LEN_TYPE + size_value as usize])
    }
}