use core::mem;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ckb_rocksdb::{ReadOptions, Transaction, TransactionDB};
use ckb_rocksdb::prelude::Put;

use crate::{Error, LenType, MetaKey, read_len_type, SIZE_LEN_TYPE, write_len_type};
use crate::redis_rocksdb::quick_list_node::QuickListNode;
use crate::redis_rocksdb::zip_list::ZipList;

struct _QuickList {
    /// node的len
    len_node: LenType,
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
    const OFFSET_META_KEY: usize = SIZE_LEN_TYPE + SIZE_LEN_TYPE;
    const OFFSET_LEFT: usize = QuickList::OFFSET_META_KEY + mem::size_of::<MetaKey>();
    const OFFSET_RIGHT: usize = QuickList::OFFSET_LEFT + mem::size_of::<MetaKey>();

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


    pub(crate) fn lpush(&mut self, tr: &Transaction<TransactionDB>, list_key: &[u8], value: &[u8]) -> Result<i32, Error> {
        let quick = self;
        if quick.len_node() == 0 {
            //可能是第一次创建，也可能是删除后，没有数据了
            let node_key = quick.next_meta_key().ok_or(Error::none_error("next_meta_key"))?;
            let mut node = QuickListNode::new();
            {
                let zip_key = quick.next_meta_key().ok_or(Error::none_error("next_meta_key"))?;
                let mut zip = ZipList::new();
                zip.set_len(1);
                zip.push_left(value.as_ref());
                tr.put(zip_key.as_ref(), zip.as_ref())?;

                node.set_len_list(1);
                node.set_len_bytes(zip.as_ref().len() as u32);
                node.set_values_key(&Some(&zip_key));
            }
            tr.put(node_key.as_ref(), node.as_ref())?;

            quick.set_len_node(1);
            quick.set_len_list(node.len_list());
            quick.set_left(&Some(&node_key));
            quick.set_right(&Some(&node_key));
            tr.put(list_key.as_ref(), quick.as_ref())?;
        } else {
            let node_key = quick.left().ok_or(Error::new("quick.left() return None".to_owned()))?.clone();

            let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(Error::new("quick.left() return None".to_owned()))?;

            // zip中的元素过多，或内存过大，都会新增加node
            if node.len_list() > QuickListNode::MAX_LEN || node.len_bytes() > QuickListNode::MAX_BYTES {
                //增加node
                let new_node_key = quick.next_meta_key().ok_or(Error::none_error("next_meta_key"))?;
                let new_node = {
                    let mut new_node = QuickListNode::new();
                    let mut zip = ZipList::new();
                    zip.set_len(1);
                    zip.push_left(value.as_ref());
                    let zip_key = quick.next_meta_key().ok_or(Error::new("next_meta_key return None".to_owned()))?;
                    tr.put(zip_key.as_ref(), zip.as_ref())?;

                    new_node.set_values_key(&Some(&zip_key));
                    new_node.set_len_list(zip.len());
                    new_node.set_len_bytes(zip.as_ref().len() as LenType);

                    new_node.set_right(&Some(&node_key));
                    node.set_left(&Some(&new_node_key));

                    new_node
                };
                tr.put(new_node_key.as_ref(), new_node.as_ref())?;
                tr.put(node_key.as_ref(), node.as_ref())?;

                quick.set_len_node(quick.len_node() + 1);
                quick.set_len_list(quick.len_list() + 1);
                quick.set_left(&Some(&new_node_key));
                tr.put(list_key.as_ref(), quick.as_ref())?;
            } else {
                let zip_key = node.values_key().ok_or(Error::none_error("values_key"))?.clone();
                let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(Error::none_error("ZipList::get"))?;
                zip.push_left(value.as_ref());

                node.set_len_list(zip.len());
                node.set_len_bytes(zip.as_ref().len() as u32);
                tr.put(zip_key.as_ref(), zip.as_ref())?;
                tr.put(node_key.as_ref(), node.as_ref())?;

                quick.set_len_list(quick.len_list() + 1);
                tr.put(list_key.as_ref(), quick.as_ref())?;
            }
        }
        Ok(quick.len_list() as i32)
    }

    pub(crate) fn rpush(&mut self, tr: &Transaction<TransactionDB>, list_key: &[u8], value: &[u8]) -> Result<i32, Error> {
        let quick = self;
        if quick.len_node() == 0 {
            //可能是第一次创建，也可能是删除后，没有数据了
            let node_key = quick.next_meta_key().ok_or(Error::none_error("next_meta_key"))?;
            let mut node = QuickListNode::new();
            {
                let zip_key = quick.next_meta_key().ok_or(Error::none_error("next_meta_key"))?;
                let mut zip = ZipList::new();
                zip.set_len(1);
                zip.push_right(value.as_ref());
                tr.put(zip_key.as_ref(), zip.as_ref())?;

                node.set_len_list(1);
                node.set_len_bytes(zip.as_ref().len() as u32);
                node.set_values_key(&Some(&zip_key));
            }
            tr.put(node_key.as_ref(), node.as_ref())?;

            quick.set_len_node(1);
            quick.set_len_list(node.len_list());
            quick.set_left(&Some(&node_key));
            quick.set_right(&Some(&node_key));
            tr.put(list_key.as_ref(), quick.as_ref())?;
        } else {
            let node_key = quick.right().ok_or(Error::new("quick.right() return None".to_owned()))?.clone();

            let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(Error::new("quick.right() return None".to_owned()))?;

            // zip中的元素过多，或内存过大，都会新增加node
            if node.len_list() > QuickListNode::MAX_LEN || node.len_bytes() > QuickListNode::MAX_BYTES {
                //增加node
                let new_node_key = quick.next_meta_key().ok_or(Error::none_error("next_meta_key"))?;
                let new_node = {
                    let mut new_node = QuickListNode::new();
                    let mut zip = ZipList::new();
                    zip.set_len(1);
                    zip.push_right(value.as_ref());
                    let zip_key = quick.next_meta_key().ok_or(Error::new("next_meta_key return None".to_owned()))?;
                    tr.put(zip_key.as_ref(), zip.as_ref())?;

                    new_node.set_values_key(&Some(&zip_key));
                    new_node.set_len_list(zip.len());
                    new_node.set_len_bytes(zip.as_ref().len() as LenType);

                    new_node.set_left(&Some(&node_key));
                    node.set_right(&Some(&new_node_key));

                    new_node
                };
                tr.put(new_node_key.as_ref(), new_node.as_ref())?;
                tr.put(node_key.as_ref(), node.as_ref())?;

                quick.set_len_node(quick.len_node() + 1);
                quick.set_len_list(quick.len_list() + 1);
                quick.set_right(&Some(&new_node_key));
                tr.put(list_key.as_ref(), quick.as_ref())?;
            } else {
                let zip_key = node.values_key().ok_or(Error::none_error("values_key"))?.clone();
                let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(Error::none_error("ZipList::get"))?;
                zip.push_right(value.as_ref());

                node.set_len_list(zip.len());
                node.set_len_bytes(zip.as_ref().len() as u32);
                tr.put(zip_key.as_ref(), zip.as_ref())?;
                tr.put(node_key.as_ref(), node.as_ref())?;

                quick.set_len_list(quick.len_list() + 1);
                tr.put(list_key.as_ref(), quick.as_ref())?;
            }
        }
        Ok(quick.len_list() as i32)
    }

    pub(crate) fn list_insert(&mut self, tr: &Transaction<TransactionDB>, list_key: &[u8], pivot: &[u8], value: &[u8],
                              f: fn(&mut ZipList,&[u8],&[u8]) -> Option<i32>) -> Result<i32, Error>{
        let mut quick = self;
        let mut node_key = quick.left().ok_or(Error::none_error("left key"))?.clone();
        let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(Error::none_error("left node"))?;

        let (zip,zip_key) = loop {
            let zip_key = node.values_key().ok_or(Error::none_error("zip key"))?;
            let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(Error::none_error("zip list"))?;
            let t = f(&mut zip,pivot.as_ref(), value.as_ref());
            if t.is_some() {
                break (Some(zip), zip_key.clone());
            }
            match node.right() {
                None => break (None, zip_key.clone()),//双向链表完成
                Some(t) => {
                    node_key = t.clone();
                    node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(Error::none_error("right node"))?;
                }
            }
        };
        let mut result = -1;
        if let Some(zip) = zip {
            node.set_len_list(zip.len());
            node.set_len_bytes(zip.as_ref().len() as LenType);
            quick.set_len_list(quick.len_list() + 1);
            result = quick.len_list() as i32;

            tr.put(zip_key, zip.as_ref())?;
            tr.put(node_key.as_ref(), node.as_ref())?;
            tr.put(list_key.as_ref(), quick.as_ref())?;
        }
        Ok(result)
    }

    pub(crate) fn next_meta_key(&mut self) -> Option<MetaKey> {
        match MetaKey::read_mut(&self.0[QuickList::OFFSET_META_KEY..]) {
            None => None,
            Some(k) => {
                k.add_sep(1);
                Some(k.clone())
            }
        }
    }

    //node的个数
    pub fn len_node(&self) -> LenType {
        read_len_type(&self.0)
    }

    pub fn set_len_node(&mut self, len: LenType) {
        write_len_type(&mut self.0, len)
    }

    //node的个数
    pub fn len_list(&self) -> LenType {
        read_len_type(&self.0[SIZE_LEN_TYPE..])
    }

    pub fn set_len_list(&mut self, len: LenType) {
        write_len_type(&mut self.0[SIZE_LEN_TYPE..], len)
    }


    #[inline]
    pub fn meta_key(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickList::OFFSET_META_KEY..])
    }

    #[inline]
    pub fn set_meta_key(&mut self, meta_key: &Option<&MetaKey>) {
        MetaKey::write(&mut self.0[QuickList::OFFSET_META_KEY..], meta_key)
    }

    #[inline]
    pub fn init_meta_key(&mut self, list_key: &[u8]) -> MetaKey {
        // todo 不能初始化两次
        let meta_key = {
            let mut meta_key = MetaKey::new();
            let mut hasher = DefaultHasher::new();
            list_key.hash(&mut hasher);
            meta_key.set_key(hasher.finish());
            meta_key
        };

        MetaKey::write(&mut self.0[QuickList::OFFSET_META_KEY..], &Some(&meta_key));
        meta_key
    }

    pub fn left(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickList::OFFSET_LEFT..])
    }

    pub fn set_left(&mut self, meta_key: &Option<&MetaKey>) {
        MetaKey::write(&mut self.0[QuickList::OFFSET_LEFT..], meta_key)
    }

    pub fn right(&self) -> Option<&MetaKey> {
        MetaKey::read(&self.0[QuickList::OFFSET_RIGHT..])
    }

    pub fn set_right(&mut self, meta_key: &Option<&MetaKey>) {
        MetaKey::write(&mut self.0[QuickList::OFFSET_RIGHT..], meta_key)
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