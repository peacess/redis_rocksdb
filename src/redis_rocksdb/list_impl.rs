use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ckb_rocksdb::prelude::{Get, Put, TransactionBegin};

use crate::{Bytes, Direction, Error, MetaKey, RedisList, RedisRocksdb};
use crate::redis_rocksdb::quick_list::QuickList;
use crate::redis_rocksdb::quick_list_node::QuickListNode;
use crate::redis_rocksdb::zip_list::ZipList;

/// [see] (https://xindoo.blog.csdn.net/article/details/109150975)
/// ssdb没有实现list，只实现了queue
///
/// redis中的list使用quicklist与ziplist实现
impl RedisList for RedisRocksdb {
    fn blpop<K: Bytes, V: Bytes>(&mut self, key: K, timeout: i64) -> Result<V, Error> {
        todo!()
    }

    fn brpop<K: Bytes, V: Bytes>(&mut self, key: K, timeout: i64) -> Result<V, Error> {
        todo!()
    }

    fn brpoplpush<K: Bytes, V: Bytes>(&mut self, srckey: K, dstkey: K, timeout: i64) -> Result<V, Error> {
        todo!()
    }

    fn lindex<K: Bytes, V: Bytes>(&self, key: K, index: i32) -> Result<V, Error> {
        todo!()
    }

    fn linsert_before<K: Bytes, P: Bytes, V: Bytes>(&mut self, key: K, pivot: P, value: V) -> Result<(), Error> {
        todo!()
    }

    fn linsert_after<K: Bytes, P: Bytes, V: Bytes>(&mut self, key: K, pivot: P, value: V) -> Result<(), Error> {
        todo!()
    }

    fn llen<K: Bytes>(&self, key: K) -> Result<i32, Error> {
        match QuickList::get(&self.db, key.as_ref())? {
            None => Ok(-1),
            Some(quick) => {
                Ok(quick.len_list() as i32)
            }
        }
    }

    fn lmove<K: Bytes, V: Bytes>(&mut self, srckey: K, dstkey: K, src_dir: Direction, dst_dir: Direction) -> Result<V, Error> {
        todo!()
    }

    fn lmpop<K: Bytes>(&mut self, numkeys: i32, key: K, dir: Direction, count: i32) {
        todo!()
    }

    fn lpop<K: Bytes, V: Bytes>(&mut self, key: K) -> Result<V, Error> {
        todo!()
    }

    fn lpush<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<i32, Error> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => QuickList::new(),
            Some(q) => q
        };

        if quick.len_list() == 0 {
            //第一次创建，没有任何数据
            let h = {
                let mut key = MetaKey::new();
                let mut hasher = DefaultHasher::new();
                key.as_ref().hash(&mut hasher);
                key.set_key(hasher.finish());
                key
            };
            let node_key = Some(h);
            quick.set_meta_key(&node_key);
            quick.set_left(&node_key);
            quick.set_right(&None);

            let mut node = QuickListNode::new();
            node.set_len_list(1);

            let zip_key = quick.next_meta_key();
            if zip_key.is_none() {
                return Err(Error::new("next_meta_key return None".to_owned()));
            }
            node.set_values_key(&zip_key);

            let mut zip = ZipList::new();
            zip.set_len(1);
            zip.push_right(value);

            node.set_len_bytes(zip.as_ref().len() as u32);

            tr.put(zip_key.as_ref(), zip.as_ref())?;
            tr.put(node_key.as_ref(), node.as_ref())?;
            tr.put(key.as_ref(), quick.as_ref())?;
        } else {
            let node_key = quick.right()?;
            let mut node =  QuickListNode::get(node_key.as_ref())??;

            /// zip中的元素过多，或内存过大，都会新增加node
            if node.len_list() > QuickListNode::MAX_LEN || node.len_bytes() > QuickListNode::MAX_BYTES {
                //增加node
            }else{

            }

            let mut node = QuickListNode::new();
            node.set_len_list(1);

            let zip_key = quick.next_meta_key();
            if zip_key.is_none() {
                return Err(Error::new("next_meta_key return None".to_owned()));
            }
            node.set_values_key(&zip_key);

            let mut zip = ZipList::new();
            zip.set_len(1);
            zip.push_right(value);

            node.set_len_bytes(zip.as_ref().len() as u32);

            tr.put(zip_key.as_ref(), zip.as_ref())?;
            tr.put(node_key.as_ref(), node.as_ref())?;
            tr.put(key.as_ref(), quick.as_ref())?;
        }
        tr.commit()?;
        Ok(0)
    }

    fn lpush_exists<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<i32, Error> {
        todo!()
        // match QuickList::get(&self.db,key.as_ref())? {
        //     None => Ok(0),
        //     Some(q) => {
        //         let right = q.right();
        //         let key = {
        //             match q.right() {
        //                 None => {
        //
        //                 }
        //                 Some(k) => {
        //                     k.as_ref()
        //                 }
        //             }
        //         };
        //
        //
        //     }
        // }
    }

    fn lrange<K: Bytes, V: Bytes>(&self, key: K, start: i32, stop: i32) -> Result<Vec<V>, Error> {
        todo!()
    }

    fn lrem<K: Bytes, V: Bytes>(&mut self, key: K, count: i32, value: V) -> Result<V, Error> {
        todo!()
    }

    fn ltrim<K: Bytes>(&mut self, key: K, start: i32, stop: i32) -> Result<i32, Error> {
        todo!()
    }

    fn lset<K: Bytes, V: Bytes>(&mut self, key: K, index: i32, value: V) {
        todo!()
    }

    fn rpop<K: Bytes>(&mut self, key: K, count: Option<i32>) {
        todo!()
    }

    fn rpoplpush<K: Bytes, V: Bytes>(&mut self, key: K, dstkey: K) -> Result<V, Error> {
        todo!()
    }

    fn rpush<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<(), Error> {
        todo!()
    }

    fn rpush_exists<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<(), Error> {
        todo!()
    }
}
