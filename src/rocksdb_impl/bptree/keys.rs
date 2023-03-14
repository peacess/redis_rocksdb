use std::mem::size_of;

use crate::{BytesType, LenType, read_int_ptr, write_int_ptr};
use crate::rocksdb_impl::bptree::node::Node;
use crate::rocksdb_impl::bptree::node_type::NodeType;

#[derive(Clone, Debug)]
pub struct Keys {
    pub number_keys: LenType,
    pub offset: isize,
    //单位是 byte, 不是key的个数
    pub bytes_number: BytesType,
}

impl Keys {
    pub const offset_data: isize = (size_of::<LenType>() + size_of::<BytesType>()) as isize;
    pub fn new() -> Self {
        Keys {
            number_keys: 0,
            offset: 0,
            bytes_number: 0,
        }
    }

    pub fn read_from(&mut self, data: &[u8], offset: isize) {
        self.offset = offset;
        unsafe {
            self.number_keys = read_int_ptr(data.as_ptr().offset(self.offset));
            self.bytes_number = read_int_ptr(data.as_ptr().offset(self.offset + size_of::<LenType>() as isize));
        }
    }

    /// offset keys中node中的偏移量，
    pub fn add(node: &mut Node, add_keys: &[&[u8]]) {
        if let NodeType::Internal(_, old_keys) = &mut node.node_type {
            let mut add_bytes = 0;
            for k in add_keys {
                add_bytes += k.len() + size_of::<BytesType>();
            }
            node.data.reserve_exact(add_bytes);
            let mut temp_b = [0 as u8; size_of::<BytesType>()];
            write_int_ptr::<BytesType>(temp_b.as_mut_ptr(), 0);
            for k in add_keys {
                write_int_ptr::<BytesType>(temp_b.as_mut_ptr(), k.len() as BytesType);
                node.data.extend_from_slice(&temp_b);
                node.data.extend_from_slice(k);
            }
            old_keys.set_number_keys(old_keys.number_keys + add_keys.len() as LenType, &mut node.data);
            old_keys.set_bytes_number(old_keys.bytes_number + add_bytes as BytesType, &mut node.data);
        } else {
            panic!("the node type is not internal")
        }
    }
    pub fn set_number_keys(&mut self, number_keys: LenType, data: &mut [u8]) {
        self.number_keys = number_keys;
        unsafe { write_int_ptr(data.as_mut_ptr().offset(self.offset), self.number_keys); }
    }
    pub fn set_bytes_number(&mut self, bytes_number: BytesType, data: &mut [u8]) {
        self.bytes_number = bytes_number;
        unsafe { write_int_ptr(data.as_mut_ptr().offset(self.offset + size_of::<LenType>() as isize), self.bytes_number); }
    }
}
