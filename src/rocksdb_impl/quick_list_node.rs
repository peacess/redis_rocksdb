use core::mem;

use rocksdb::{Transaction, TransactionDB};

use crate::{read_len_type, write_len_type, LenType, MetaKey, RrError, BYTES_LEN_TYPE};

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
    pub const MAX_LEN: LenType = 124;
    pub const MAX_BYTES: LenType = QuickListNode::MAX_LEN * 4;

    const OFFSET_LEFT: usize = BYTES_LEN_TYPE + BYTES_LEN_TYPE;
    const OFFSET_RIGHT: usize = QuickListNode::OFFSET_LEFT + mem::size_of::<MetaKey>();
    const OFFSET_VALUES_KEY: usize = QuickListNode::OFFSET_RIGHT + mem::size_of::<MetaKey>();
    pub fn new() -> Self {
        QuickListNode([0; mem::size_of::<_QuickListNode>()])
    }

    pub(crate) fn get(tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<QuickListNode>, RrError> {
        let v = tr.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                if v.len() == mem::size_of::<QuickListNode>() {
                    let t: [u8; mem::size_of::<QuickListNode>()] = v.to_vec().as_slice().try_into()?;
                    Ok(Some(QuickListNode::from(t)))
                } else {
                    Err(RrError::message("can not convert vec to QuickListNode, the len is not eq".to_owned()))
                }
            }
        }
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
        read_len_type(&self.0[BYTES_LEN_TYPE..])
    }

    pub fn set_len_bytes(&mut self, len: LenType) {
        write_len_type(&mut self.0[BYTES_LEN_TYPE..], len)
    }

    pub fn left(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickListNode::OFFSET_LEFT..])
    }

    pub fn set_left(&mut self, meta_key: &Option<&MetaKey>) {
        MetaKey::write(&mut self.0[QuickListNode::OFFSET_LEFT..], meta_key)
    }

    pub fn right(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickListNode::OFFSET_RIGHT..])
    }

    pub fn set_right(&mut self, meta_key: &Option<&MetaKey>) {
        MetaKey::write(&mut self.0[QuickListNode::OFFSET_RIGHT..], meta_key)
    }

    pub fn values_key(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickListNode::OFFSET_VALUES_KEY..])
    }

    pub fn set_values_key(&mut self, meta_key: &Option<&MetaKey>) {
        MetaKey::write(&mut self.0[QuickListNode::OFFSET_VALUES_KEY..], meta_key)
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
