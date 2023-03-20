use std::mem::size_of;
use anyhow::Error;

use crate::{BytesType, LenType, read_int_ptr};
use crate::rocksdb_impl::bptree::node::Node;

/// 数据直接使用kv存入数据库中，所以leaf节点只有key的内容
#[derive(Clone, Debug)]
pub struct LeafData {
    pub number_keys: LenType,
    //单位是 byte, 不是key的个数
    pub bytes_number: BytesType,

    pub offset: isize,

}

impl LeafData {
    pub fn new() -> Self {
        LeafData {
            number_keys: 0,
            offset: 0,
            bytes_number: 0,
        }
    }

    pub fn read_from(&mut self, data: &[u8]) {
        self.offset = Node::offset_node_data as isize;
        unsafe {
            self.number_keys = read_int_ptr(data.as_ptr().offset(self.offset));
            self.bytes_number = read_int_ptr(data.as_ptr().offset(self.offset + size_of::<LenType>() as isize));
        }
    }

    pub fn binary_search(&self, data:&[u8], key: &[u8]) -> Result<(), Error> {
        Ok(())
    }
}