use core::mem;

use crate::{LenType, MetaKey, read_len_type, SIZE_LEN_TYPE, write_len_type};

///
/// ```rust
/// use redis_rocksdb::{LenType, MetaKey};
///
/// struct QuickListNode{
///     len_list: LenType,
///     len_bytes: LenType,
///     left: Option<MetaKey>,
///     right: Option<MetaKey>,
///     value_key: Option<MetaKey>,
/// }
/// ```

struct _QuickListNode {
    len_list: LenType,
    len_bytes: LenType,
    left: Option<MetaKey>,
    right: Option<MetaKey>,
    values_key: Option<MetaKey>,
}

pub(crate) struct QuickListNode([u8; mem::size_of::<_QuickListNode>()]);

impl QuickListNode {
    const offset_left: usize = SIZE_LEN_TYPE + SIZE_LEN_TYPE;
    const offset_right: usize = QuickListNode::offset_left + mem::size_of::<MetaKey>();
    const offset_values_key: usize = QuickListNode::offset_right + mem::size_of::<MetaKey>();
    pub fn new() -> Self {
        QuickListNode([0; mem::size_of::<_QuickListNode>()])
    }

    //计算在 ziplist中value个数
    pub fn len_list(&self) -> LenType {
        read_len_type(&self.0)
    }

    pub fn set_len_list(&mut self, len: LenType) {
        write_len_type(&mut self.0, len)
    }

    //在 ziplist的bytes
    pub fn len_bytes(&self) -> LenType {
        read_len_type(&self.0[SIZE_LEN_TYPE..])
    }

    pub fn set_len_bytes(&mut self, len: LenType) {
        write_len_type(&mut self.0[SIZE_LEN_TYPE..], len)
    }

    pub fn left(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickListNode::offset_left..])
    }

    pub fn set_left(&mut self, meta_key: &Option<MetaKey>) {
        MetaKey::write(&mut self.0[QuickListNode::offset_left..], meta_key)
    }

    pub fn right(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickListNode::offset_right..])
    }

    pub fn set_right(&mut self, meta_key: &Option<MetaKey>) {
        MetaKey::write(&mut self.0[QuickListNode::offset_right..], meta_key)
    }

    pub fn values_key(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickListNode::offset_values_key..])
    }

    pub fn set_values_key(&mut self, meta_key: &Option<MetaKey>) {
        MetaKey::write(&mut self.0[QuickListNode::offset_values_key..], meta_key)
    }
}


impl From<[u8; mem::size_of::<_QuickListNode>()]> for QuickListNode {
    fn from(bytes: [u8; mem::size_of::<_QuickListNode>()]) -> Self {
        QuickListNode(bytes)
    }
}

impl AsRef<[u8]> for QuickListNode {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
