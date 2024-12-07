use std::convert::From;

use crate::{
    datas::{KeyMetas, VecBytes},
    rocksdb_impl::bptree::{children::Children, leaf_data::LeafData},
};

pub type Keys = VecBytes<KeyMetas>;

// NodeType Represents different node types in the BTree.
#[derive(Clone, Debug)]
pub enum NodeType {
    /// Internal nodes contain a vector of pointers to their children and a vector of keys.
    Internal(Children, Keys),

    /// Leaf nodes contain a vector of Keys and values.
    Leaf(LeafData),

    None,
}

// Converts a byte to a NodeType.
impl From<u8> for NodeType {
    fn from(orig: u8) -> NodeType {
        match orig {
            0x00 => NodeType::None,
            0x01 => NodeType::Internal(Children::new(), Keys::new()),
            0x02 => NodeType::Leaf(LeafData::new()),
            _ => NodeType::None,
        }
    }
}

// Converts a NodeType to a byte.
impl From<&NodeType> for u8 {
    fn from(orig: &NodeType) -> u8 {
        match orig {
            NodeType::None => 0x00,
            NodeType::Internal(_, _) => 0x01,
            NodeType::Leaf(_) => 0x02,
        }
    }
}
