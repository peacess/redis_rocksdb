use core::{mem, ptr};

use ckb_rocksdb::prelude::{Get, Put};
use ckb_rocksdb::ReadOptions;

use crate::{EndianScalar, Error, LenType, MetaKey, read_int, read_len_type, RedisRocksdb, SIZE_LEN_TYPE, write_len_type};
use crate::redis_rocksdb::quick_list_node::QuickListNode;

struct _QuickList {
    /// list的len
    len_list: LenType,
    /// 用于产生下一个 node的 meta key
    meta_key: MetaKey,
    left: Option<MetaKey>,
    /// 当只有一个node时， right == left
    right: Option<MetaKey>,
    // //todo 对于大的list的read进行优化
    // index_middle: LenType,
    // middle: Option<MetaKey>,
}

pub struct QuickList([u8; mem::size_of::<_QuickList>()]);

impl QuickList {
    const offset_meta_key: usize = SIZE_LEN_TYPE;
    const offset_left: usize = QuickList::offset_meta_key + mem::size_of::<MetaKey>();
    const offset_right: usize = QuickList::offset_left + mem::size_of::<MetaKey>();

    pub fn new() -> Self {
        QuickList([0; mem::size_of::<_QuickList>()])
    }

    pub(crate) fn get<T: ckb_rocksdb::ops::Get<ReadOptions>>(db: &T, key: &[u8]) -> Result<Option<QuickList>, Error> {
        let v = db.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                if v.len() == mem::size_of::<QuickList>() {
                    let t: [u8; mem::size_of::<QuickList>()] = v.to_vec().as_slice().try_into()?;
                    Ok(Some(QuickList::from(t)))
                } else {
                    Err(Error::new("can not convert vec to QuickList, the len is not eq".to_owned()))
                }
            }
        }
    }

    pub(crate) fn get_node<T: ckb_rocksdb::ops::Get<ReadOptions>>(db: &T, key: &[u8]) -> Result<Option<QuickListNode>, Error> {
        let v = db.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                if v.len() == mem::size_of::<QuickListNode>() {
                    let t: [u8; mem::size_of::<QuickListNode>()] = v.to_vec().as_slice().try_into()?;
                    Ok(Some(QuickListNode::from(t)))
                } else {
                    Err(Error::new("can not convert vec to QuickList, the len is not eq".to_owned()))
                }
            }
        }
    }

    pub(crate) fn next_meta_key(&mut self) -> Option<MetaKey> {
        match MetaKey::read_mut(&self.0[QuickList::offset_meta_key..]) {
            None => None,
            Some(k) =>{
                k.add_sep(1);
                Some(k.clone())
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


    #[inline]
    pub fn meta_key(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickList::offset_meta_key..])
    }

    #[inline]
    pub fn set_meta_key(&mut self, meta_key: &Option<MetaKey>) {
        MetaKey::write(&mut self.0[QuickList::offset_meta_key..], meta_key)
    }

    pub fn left(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickList::offset_left..])
    }

    pub fn set_left(&mut self, meta_key: &Option<MetaKey>) {
        MetaKey::write(&mut self.0[QuickList::offset_left..], meta_key)
    }

    pub fn right(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickList::offset_right..])
    }

    pub fn set_right(&mut self, meta_key: &Option<MetaKey>) {
        MetaKey::write(&mut self.0[QuickList::offset_right..], meta_key)
    }
}


impl From<[u8; mem::size_of::<QuickList>()]> for QuickList {
    fn from(bytes: [u8; mem::size_of::<QuickList>()]) -> Self {
        QuickList(bytes)
    }
}

impl AsRef<[u8]> for QuickList {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
