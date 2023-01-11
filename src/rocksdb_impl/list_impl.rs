use anyhow::Context;
use ckb_rocksdb::prelude::{Delete, Put, TransactionBegin};

use crate::{Bytes, LenType, RedisList, RedisRocksdb, RrError};
use crate::rocksdb_impl::quick_list::QuickList;
use crate::rocksdb_impl::quick_list_node::QuickListNode;
use crate::rocksdb_impl::zip_list::ZipList;

/// [see] (https://xindoo.blog.csdn.net/article/details/109150975)
/// ssdb没有实现list，只实现了queue
///
/// redis中的list使用quicklist与ziplist实现
impl RedisList for RedisRocksdb {
    fn blpop<K: Bytes, V: Bytes>(&mut self, key: &K, timeout: i64) -> Result<V, RrError> {
        todo!()
    }

    fn brpop<K: Bytes, V: Bytes>(&mut self, key: &K, timeout: i64) -> Result<V, RrError> {
        todo!()
    }

    fn brpoplpush<K: Bytes, V: Bytes>(&mut self, srckey: &K, dstkey: &K, timeout: i64) -> Result<V, RrError> {
        todo!()
    }

    fn lindex<K: Bytes>(&self, key: &K, index: i32) -> Result<Vec<u8>, RrError> {
        let t = QuickList::get(&self.db, key.as_ref())?.ok_or(RrError::not_find("key of list"))?;
        if index >= t.len_list() as i32 {
            return Err(RrError::not_find(&format!("the index {}", index)));
        }
        //todo read only
        let tr = self.db.transaction_default();
        let node_key = t.left().context("left of quick list")?;
        let mut node = QuickListNode::get(&tr, node_key.as_ref())?.context("left node")?;
        let mut it_index = 0i32;
        it_index += node.len_list() as i32;
        while index >= it_index {
            let next_key = node.right().context("right node")?;
            node = QuickListNode::get(&tr, next_key.as_ref())?.context("next node")?;
            it_index += node.len_list() as i32;
        }

        let value_key = node.values_key().context("value key")?;
        let zip = ZipList::get(&tr, value_key.as_ref())?.context("zip list")?;
        let zip_index = index - (it_index - node.len_list() as i32);
        let v = zip.index(zip_index).ok_or(RrError::not_find(&format!("the index {}", index)))?;
        tr.commit()?;
        Ok(v.to_vec())
    }
    fn linsert_before<K: Bytes, V: Bytes>(&mut self, key: &K, pivot: &V, value: &V) -> Result<i32, RrError> {
        let mut quick = {
            match QuickList::get(&self.db, key.as_ref())? {
                None => return Ok(0),
                Some(q) => q
            }
        };
        let tr = self.db.transaction_default();
        let result = quick.list_insert(&tr, key.as_ref(), pivot.as_ref(), value.as_ref(), ZipList::insert_value_left)?;
        tr.commit()?;
        Ok(result)
    }

    fn linsert_after<K: Bytes, V: Bytes>(&mut self, key: &K, pivot: &V, value: &V) -> Result<i32, RrError> {
        let mut quick = {
            match QuickList::get(&self.db, key.as_ref())? {
                None => return Ok(0),
                Some(q) => q
            }
        };

        let tr = self.db.transaction_default();
        let result = quick.list_insert(&tr, key.as_ref(), pivot.as_ref(), value.as_ref(), ZipList::insert_value_right)?;
        tr.commit()?;
        Ok(result)
    }

    fn llen<K: Bytes>(&self, key: &K) -> Result<i32, RrError> {
        match QuickList::get(&self.db, key.as_ref())? {
            None => Ok(-1),
            Some(quick) => {
                Ok(quick.len_list() as i32)
            }
        }
    }

    fn lpop<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<u8>>, RrError> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => return Ok(None),
            Some(q) => q
        };
        if quick.len_list() < 1 {
            return Ok(None);
        }
        let node_key = quick.left().ok_or(RrError::none_error("left key"))?.clone();
        let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("left node"))?;
        let zip_key = node.values_key().ok_or(RrError::none_error("zip key"))?.clone();
        let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(RrError::none_error("zip list"))?;
        let value = zip.pop_left();

        if zip.len() == 0 {
            //没有数据，删除quick list node
            if quick.len_node() == 1 {
                tr.delete(zip_key)?;
                tr.delete(node_key)?;
                quick.set_right(&None);
                quick.set_left(&None);
                quick.set_len_list(0);
                quick.set_len_node(0);
                tr.put(key.as_ref(), quick)?;
            } else {
                let left = node.right();
                quick.set_right(&node.left());
                quick.set_len_node(quick.len_node() - 1);
                quick.set_len_list(quick.len_list() - 1);
                tr.delete(zip_key)?;
                tr.delete(node_key)?;
                tr.put(key.as_ref(), quick)?;
            }
        } else {
            node.set_len_list(zip.len());
            node.set_len_bytes(zip.as_ref().len() as u32);
            quick.set_len_list(quick.len_list() - 1);

            tr.put(zip_key, zip)?;
            tr.put(node_key, node.as_ref())?;
        }

        tr.commit()?;
        Ok(value)
    }

    fn lpush<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i32, RrError> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => {
                let mut q = QuickList::new();
                q.init_meta_key(key.as_ref());
                q
            }
            Some(q) => q
        };
        let re = quick.lpush(&tr, key.as_ref(), value.as_ref())?;
        tr.commit()?;
        Ok(re)
    }

    fn lpush_exists<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i32, RrError> {
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => return Ok(0),
            Some(q) => q
        };
        let tr = self.db.transaction_default();
        let re = quick.lpush(&tr, key.as_ref(), value.as_ref())?;
        tr.commit()?;
        Ok(re)
    }


    fn lrange<K: Bytes>(&self, key: &K, start: i32, stop: i32) -> Result<Vec<Vec<u8>>, RrError> {
        let mut result = Vec::new();
        let quick = match QuickList::get(&self.db, key.as_ref())? {
            None => return Ok(result),
            Some(q) => q
        };
        if quick.len_list() < 1 {
            return Ok(result);
        }

        let start_index = ZipList::count_index(quick.len_list() as i32, start) as usize;
        let stop_index = ZipList::count_index(quick.len_list() as i32, stop) as usize;
        if start_index > stop_index {
            return Ok(result);
        }

        //todo read only
        let tr = self.db.transaction_default();

        let node_key = quick.left().ok_or(RrError::none_error("left key"))?;
        let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("quick list node"))?;
        let offset = 0usize;
        loop {
            let len_zip = node.len_list();
            if start_index < len_zip as usize + offset {
                let temp = ZipList::count_in_index(len_zip, offset, start_index, stop_index);
                if let Some((start_in, stop_in)) = temp {
                    let zip_key = node.values_key().ok_or(RrError::none_error("zip key"))?;
                    let zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(RrError::none_error("zip"))?;
                    let one = zip.range(start_in as i32, stop_in as i32);
                    result.extend(one);
                }//else 是没有数据
            }

            if stop_index < len_zip as usize + offset {
                //取了所有数据
                break;
            }

            if let Some(t) = node.right() {
                node = QuickListNode::get(&tr, t.as_ref())?.ok_or(RrError::none_error("quick list node"))?;
            } else {
                // 没有更多的节点
                break;
            }
        }

        tr.commit()?;
        Ok(result)
    }

    fn lrem<K: Bytes, V: Bytes>(&mut self, list_key: &K, count: i32, value: &V) -> Result<LenType, RrError> {
        let mut quick = match QuickList::get(&self.db, list_key.as_ref())? {
            None => return Ok(0),
            Some(q) => q
        };

        let mut rem_count = 0u32;

        let tr = self.db.transaction_default();

        if count > 0 { //正向遍历
            let count = count as usize;
            let mut node_key = quick.left().ok_or(RrError::none_error("left key"))?.clone();
            let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("left node"))?;

            loop {
                let zip_key = node.values_key().ok_or(RrError::none_error("zip key"))?.clone();
                let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(RrError::none_error("zip"))?;

                let done = zip.rem((count - rem_count as usize) as i32, value.as_ref());
                rem_count += done;

                if done != 0 {
                    quick.modify_node(&tr, list_key.as_ref(), zip_key.as_ref(), &mut zip, node_key.as_ref(), &mut node)?;
                }

                if rem_count == count as u32 {
                    break;
                }
                if let Some(t) = node.right() {
                    node_key = t.clone();
                } else {
                    break;
                }
                node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("right node"))?;
            }
        } else if count < 0 { //反向遍历
            let count = count.abs() as usize;
            let mut node_key = quick.right().ok_or(RrError::none_error("left key"))?.clone();
            let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("node"))?;

            loop {
                let zip_key = node.values_key().ok_or(RrError::none_error("zip key"))?.clone();
                let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(RrError::none_error("zip"))?;

                let mut will_count = count as i32 - rem_count as i32;
                let done = zip.rem(-will_count, value.as_ref());
                rem_count += done;

                if done != 0 {
                    quick.modify_node(&tr, list_key.as_ref(), zip_key.as_ref(), &mut zip, node_key.as_ref(), &mut node)?;
                }

                if rem_count == count as u32 {
                    break;
                }
                if let Some(t) = node.left() {
                    node_key = t.clone();
                } else {
                    break;
                }
                node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("right node"))?;
            }
        } else { //正向删除所有相等的值
            let count = count as usize;
            let mut node_key = quick.left().ok_or(RrError::none_error("left key"))?.clone();
            let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("left node"))?;

            loop {
                let zip_key = node.values_key().ok_or(RrError::none_error("zip key"))?.clone();
                let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(RrError::none_error("zip"))?;

                let done = zip.rem((count - rem_count as usize) as i32, value.as_ref());
                rem_count += done;

                if done != 0 {
                    quick.modify_node(&tr, list_key.as_ref(), zip_key.as_ref(), &mut zip, node_key.as_ref(), &mut node)?;
                }
                if let Some(t) = node.right() {
                    node_key = t.clone();
                } else {
                    break;
                }
                node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("right node"))?;
            }
        }


        if rem_count > 0 {
            quick.set_len_list(quick.len_list() - rem_count);
            tr.put(list_key, quick)?;
        }

        tr.commit()?;
        Ok(rem_count)
    }

    fn ltrim<K: Bytes>(&mut self, key: K, start: i32, stop: i32) -> Result<i32, RrError> {
        todo!()
    }

    fn lset<K: Bytes, V: Bytes>(&mut self, key: &K, index: i32, value: &V) -> Result<Vec<u8>, RrError> {
        let t = QuickList::get(&self.db, key.as_ref())?.ok_or(RrError::not_find("key of list"))?;
        if index >= t.len_list() as i32 || index < 0 {
            return Err(RrError::not_find(&format!("the index {}", index)));
        }
        //todo read only
        let tr = self.db.transaction_default();
        let node_key = t.left().context("left of quick list")?;
        let mut node = QuickListNode::get(&tr, node_key.as_ref())?.context("left node")?;
        let mut it_index = 0i32;
        it_index += node.len_list() as i32;
        while index >= it_index {
            let next_key = node.right().context("right node")?;
            node = QuickListNode::get(&tr, next_key.as_ref())?.context("next node")?;
            it_index += node.len_list() as i32;
        }

        let value_key = node.values_key().context("value key")?;
        let mut zip = ZipList::get(&tr, value_key.as_ref())?.context("zip list")?;
        let zip_index = index - (it_index - node.len_list() as i32);
        let v = zip.set(zip_index, value.as_ref()).ok_or(RrError::not_find(&format!("the index {}", index)))?;
        tr.put(value_key, zip)?;
        tr.commit()?;
        Ok(v)
    }


    fn rpop<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<u8>>, RrError> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => return Ok(None),
            Some(q) => q
        };
        if quick.len_list() < 1 {
            return Ok(None);
        }
        let node_key = quick.right().ok_or(RrError::none_error("right key"))?.clone();
        let mut node = QuickListNode::get(&tr, node_key.as_ref())?.ok_or(RrError::none_error("right node"))?;
        let zip_key = node.values_key().ok_or(RrError::none_error("zip key"))?.clone();
        let mut zip = ZipList::get(&tr, zip_key.as_ref())?.ok_or(RrError::none_error("zip list"))?;
        let value = zip.pop_right();

        if zip.len() == 0 {
            //没有数据，删除quick list node
            if quick.len_node() == 1 {
                tr.delete(zip_key)?;
                tr.delete(node_key)?;
                quick.set_right(&None);
                quick.set_left(&None);
                quick.set_len_list(0);
                quick.set_len_node(0);
                tr.put(key.as_ref(), quick)?;
            } else {
                let left = node.left();
                quick.set_right(&node.left());
                quick.set_len_node(quick.len_node() - 1);
                quick.set_len_list(quick.len_list() - 1);
                tr.delete(zip_key)?;
                tr.delete(node_key)?;
                tr.put(key.as_ref(), quick)?;
            }
        } else {
            node.set_len_list(zip.len());
            node.set_len_bytes(zip.as_ref().len() as u32);
            quick.set_len_list(quick.len_list() - 1);

            tr.put(zip_key.as_ref(), zip)?;
            tr.put(node_key.as_ref(), node.as_ref())?;
        }

        tr.commit()?;
        Ok(value)
    }

    fn rpoplpush<K: Bytes, V: Bytes>(&mut self, key: &K, dstkey: &K) -> Result<V, RrError> {
        todo!()
    }

    fn rpush<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i32, RrError> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => {
                let mut q = QuickList::new();
                q.init_meta_key(key.as_ref());
                q
            }
            Some(q) => q
        };
        let re = quick.rpush(&tr, key.as_ref(), value.as_ref())?;
        tr.commit()?;
        Ok(re)
    }

    fn rpush_exists<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i32, RrError> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => return Ok(0),
            Some(q) => q
        };
        let re = quick.rpush(&tr, key.as_ref(), value.as_ref())?;
        tr.commit()?;
        Ok(re)
    }

    fn l_clear<K: Bytes>(&mut self, key: &K) -> Result<i32, RrError> {
        let tr = self.db.transaction_default();
        let mut quick = match QuickList::get(&self.db, key.as_ref())? {
            None => return Ok(0),
            Some(q) => q
        };
        let re = quick.len_node();
        quick.clear(&tr, key.as_ref())?;
        tr.commit()?;
        Ok(re as i32)
    }
}
