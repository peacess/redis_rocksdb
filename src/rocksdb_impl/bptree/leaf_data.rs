use std::{mem, mem::size_of};

use crate::{
    datas::{BytesMeta, Metas, VecBytes},
    read_int_ptr, write_int_ptr, BytesType, LenType,
};

/// 数据直接使用kv存入数据库中，所以leaf节点只有key的内容
pub type LeafData = VecBytes<KeyValueMetas>;

pub struct KeyValue {}

impl KeyValue {
    pub fn to_vec(key: &[u8], value: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(key.len() + value.len() + size_of::<BytesType>());
        // let mut remaining_bytes = bytes.spare_capacity_mut();
        unsafe {
            write_int_ptr(bytes.as_mut_ptr(), key.len() as BytesType);
            bytes.set_len(size_of::<BytesType>());
        }
        bytes.extend_from_slice(key);
        bytes.extend_from_slice(value);
        bytes
    }
    pub fn to_kv(data: &[u8]) -> (&[u8], &[u8]) {
        let l = read_int_ptr::<BytesType>(data.as_ptr());
        let key = &data[size_of::<BytesType>()..size_of::<BytesType>() + l as usize];
        let value = &data[size_of::<BytesType>() + l as usize..];
        (key, value)
    }
}

#[derive(Debug, Clone)]
pub struct KeyValueMetas {
    //这个数据是一个引用
    data: Vec<BytesMeta>,
}

impl Drop for KeyValueMetas {
    fn drop(&mut self) {
        let data = std::mem::take(&mut self.data);
        mem::forget(data);
    }
}

impl Metas for KeyValueMetas {
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
                key.cmp(o_key)
            })
        }
    }
}
