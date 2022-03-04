use std::mem;

use crate::{EndianScalar, MetaKey};

struct QuickList {
    /// node的数量
    node_size: i16,
    /// list的len
    values_size: i32,
    /// 用于产生下一个 node的 meta key
    meta_key: MetaKey,
    left: Option<MetaKey>,
    right: Option<MetaKey>,
    middle: Option<MetaKey>,
}

///
/// ```rust
/// use redis_rocksdb::MetaKey;
///
/// struct QuickListNode{
///     value_size: i32,
///     left: Option<MetaKey>,
///     right: Option<MetaKey>,
///     len_value: i32,
///     value: &[u8],
/// }
/// ```
struct QuickListNode(Vec<u8>);

impl From<Vec<u8>> for QuickListNode {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes;
        if bytes.len() < QuickListNode::len_init {
            bytes.resize(QuickListNode::len_init, 0);
        }
        QuickListNode(bytes)
    }
}

impl AsRef<[u8]> for QuickListNode {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl QuickListNode {
    const len_init: usize = 32;
    const len_value_size: usize = mem::size_of::<i32>();
    const offset_left: usize = QuickListNode::len_value_size;
    const offset_right: usize = QuickListNode::offset_left + mem::size_of::<MetaKey>();
    pub fn new() -> Self {
        QuickListNode(Vec::from([0; QuickListNode::len_init]))
    }

    pub fn memory_size(&self) -> i32 {
        self.0.len() as i32
    }

    pub fn value_size(&self) -> i32 {
        let mut mem = core::mem::MaybeUninit::<i32>::uninit();
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.0[..QuickListNode::len_value_size].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                core::mem::size_of::<i32>(),
            );
            mem.assume_init()
        }.from_little_endian()
    }

    pub fn set_value_size(&mut self, value_size: i32) {
        let x_le = value_size.to_little_endian();
        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const i32 as *const u8,
                self.0[..QuickListNode::len_value_size].as_mut_ptr(),
                QuickListNode::len_value_size,
            );
        }
    }

    pub fn left(&self) -> Option<&MetaKey> {
        const Len: usize = mem::size_of::<MetaKey>();
        let t = &self.0[QuickListNode::offset_left..QuickListNode::offset_left + Len];
        let zero_ = [0u8; Len].as_slice();
        match t {
            zero_ => None,
            _ => {
                Some(unsafe { &*(t.as_ptr() as *const MetaKey) })
            }
        }
    }

    pub fn right(&self) -> Option<&MetaKey> {
        const Len: usize = mem::size_of::<MetaKey>();
        let t = &self.0[QuickListNode::offset_right..QuickListNode::offset_right + Len];
        let zero_ = [0u8; Len].as_slice();
        match t {
            zero_ => None,
            _ => {
                Some(unsafe { &*(t.as_ptr() as *const MetaKey) })
            }
        }
    }
}


struct ZipList<'a>(&'a [u8]);