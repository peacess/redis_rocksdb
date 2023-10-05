use std::mem::size_of;

use crate::{BytesType, LenType, read_int_ptr, write_int_ptr};
use crate::rocksdb_impl::bptree::db_key::DbKey;
use crate::rocksdb_impl::bptree::node::Node;
use crate::rocksdb_impl::bptree::node_type::NodeType;

#[derive(Clone, Debug)]
pub struct Children {
    pub number_children: LenType,
    // //单位是 byte, 不是key的个数
    pub bytes_number: BytesType,
    pub offset: isize,
}

impl From<&[u8]> for Children {
    fn from(data: &[u8]) -> Self {
        let offset = Node::offset_node_data as isize;
        let number_children = read_int_ptr(unsafe { data.as_ptr().offset(offset) });
        let bytes_number = read_int_ptr(unsafe { data.as_ptr().offset(offset + size_of::<LenType>() as isize) });
        Self {
            offset,
            number_children,
            bytes_number,
        }
    }
}

impl Children {
    pub const offset_data: isize = (size_of::<LenType>() + size_of::<BytesType>()) as isize;
    pub fn new() -> Self {
        Children {
            number_children: 0,
            offset: Node::offset_node_data as isize,
            bytes_number: 0,
        }
    }
    pub fn read_from(&mut self, data: &[u8]) {
        self.offset = Node::offset_node_data as isize;
        unsafe {
            self.number_children = read_int_ptr(data.as_ptr().offset(self.offset));
            self.bytes_number = read_int_ptr(data.as_ptr().offset(self.offset + size_of::<LenType>() as isize));
        }
    }

    pub fn get(&self, data: &[u8], index: usize) -> DbKey {
        let start = self.offset as usize + Children::offset_data as usize + index * DbKey::LEN_DB_KEY;
        DbKey::from(&data[start..])
    }

    pub fn set_number_children(&mut self, number_children: LenType, data: &mut [u8]) {
        self.number_children = number_children;
        self.bytes_number = self.number_children as BytesType * DbKey::LEN_DB_KEY as BytesType;
        unsafe {
            write_int_ptr(data.as_mut_ptr().offset(self.offset), self.number_children);
            write_int_ptr(data.as_mut_ptr().offset(self.offset + size_of::<LenType>() as isize), self.bytes_number);
        }
    }

    pub fn offset_keys(&self) -> isize {
        self.offset + self.bytes_number as isize + Children::offset_data as isize
    }

    pub fn add(node: &mut Node, children: &[&[u8]]) -> Children {
        if let NodeType::Internal(old_children, _) = &mut node.node_type {
            let mut new_bytes = DbKey::LEN_DB_KEY * children.len();
            node.data.reserve_exact(new_bytes);
            unsafe {//移动keys的数据
                let old = old_children.number_children;
                let offset_keys = old_children.offset + Children::offset_data + old_children.bytes_number as isize;
                std::ptr::copy(node.data.as_mut_ptr().offset(offset_keys),
                               node.data.as_mut_ptr().offset(offset_keys + new_bytes as isize),
                               node.data.len() - offset_keys as usize - new_bytes);
                node.data.set_len(node.data.len() + new_bytes);
            }
            let mut offset_keys = old_children.offset + Children::offset_data + old_children.bytes_number as isize;
            for k in children {
                unsafe { std::ptr::copy_nonoverlapping(k.as_ptr(), node.data.as_mut_ptr().offset(offset_keys), k.len()); }
                offset_keys += k.len() as isize;
            }
            old_children.set_number_children(old_children.number_children + children.len() as LenType, &mut node.data);
            old_children.clone()
        } else {
            panic!("the node type is not Internal");
        }
    }
}