use core::slice::SlicePattern;
use std::mem::copy;
use std::ptr;
use anyhow::Context;
use ckb_rocksdb::prelude::{Delete, Get, Put, TransactionBegin};

use crate::{Bytes, LenType, read_int, RedisList, RedisRocksdb, RrError, Stack, write_int};
use crate::rocksdb_impl::quick_list::QuickList;
use crate::rocksdb_impl::quick_list_node::QuickListNode;
use crate::rocksdb_impl::zip_list::ZipList;


impl Stack for RedisRocksdb {
    fn index<K: Bytes>(&self, key: &K, index: i64) -> Result<Vec<u8>, RrError> {
        let stack = StackHeader::get_stack(&self.db, key)?;
        match stack {
            None => Ok(vec![]),
            Some(v) => {

            }
        }
    }

    fn len<K: Bytes>(&self, key: &K) -> Result<i64, RrError> {
        todo!()
    }

    fn push<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i64, RrError> {
        todo!()
    }

    fn pushs<K: Bytes, V: Bytes>(&mut self, key: &K, values: &[&V]) -> Result<i64, RrError> {
        todo!()
    }

    fn push_exists<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i64, RrError> {
        todo!()
    }

    fn range<K: Bytes>(&self, key: &K, start: i64, stop: i64) -> Result<Vec<Vec<u8>>, RrError> {
        todo!()
    }

    fn set<K: Bytes, V: Bytes>(&mut self, key: &K, index: i64, value: &V) -> Result<Vec<u8>, RrError> {
        todo!()
    }

    fn pop<K: Bytes>(&self, key: &K) -> Result<Vec<u8>, RrError> {
        todo!()
    }

    fn pops<K: Bytes>(&self, key: &K, amount: u64) -> Result<Vec<Vec<u8>>, RrError> {
        todo!()
    }

    fn poplpush<K: Bytes, V: Bytes>(&mut self, key: &K, dstkey: &K) -> Result<V, RrError> {
        todo!()
    }

    fn clear<K: Bytes>(&mut self, key: &K) -> Result<i64, RrError> {
        todo!()
    }
}

struct StackHeader<'a> {
    size: [u8; 8],
    // key: &'a [u8],
}

impl StackHeader {

    fn size_i64(&self) -> i64 {
        read_int(&self.size)
    }
    fn add_1(&mut self) -> i64{
        let mut t = read_int(&self.size);
        t +=1;
        write_int(&mut self.size, t);
        t
    }

    fn add(&mut self, v: i64) -> i64{
        let mut t = read_int(&self.size);
        t +=v;
        write_int(&mut self.size, t);
        t
    }

    fn make_key_index(key: &[u8], index: i64) -> Vec<u8> {
        let mut v = Vec::with_capacity(key.len() + 9);
        unsafe {
            v.set_len(v.capacity());
            ptr::copy(key.as_ptr(), v.as_mut_ptr(), key.len());
        }
        let mut vv = v.as_mut_slice();
        vv[key.len()] = b'_';
        write_int(&mut vv[key.len() + 1..], index);
        v
    }

    fn get_stack(db: &ckb_rocksdb::TransactionDB, key: &[u8]) -> Result<Option<Self>, RrError>{
        let v = db.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                if v.len() < 8 {
                    Err(RrError::data_error("key < 8"))
                }else{
                    OK(Some(StackHeader::from(v.as_slice())))
                }
            }
        }
    }
    fn get_index(&self,db: &ckb_rocksdb::TransactionDB, key: &[u8], index: i64) -> Result<Vec<u8>, RrError>{
        let v = db.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                if v.len() < 8 {
                    Err(RrError::data_error("key < 8"))
                }else{
                    OK(Some(StackHeader::from(v.as_slice())))
                }
            }
        }
    }
}

impl From<i64> for StackHeader{
    fn from(value: i64) -> Self {
        let mut s = StackHeader{ size: [u8;8] };
        write_int(&mut s.size, value);
        s
    }
}

impl From<&[u8]> for StackHeader{
    fn from(value: &[u8]) -> Self {
        let mut s = StackHeader{ size: [u8;8] };
        s.size.copy_from_slice(value);
        s
    }
}

impl AsRef<[u8]> for StackHeader {
    fn as_ref(&self) -> &[u8] {
        &self.size
    }
}

impl Bytes for StackHeader {
}
