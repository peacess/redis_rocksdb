use std::ops::Deref;
use std::ptr;

use ckb_rocksdb::prelude::Get;

use crate::{Bytes, read_int, RedisRocksdb, RrError, Stack, write_int};

impl Stack for RedisRocksdb {
    fn index<K: Bytes>(&self, key: &K, index: i64) -> Result<Vec<u8>, RrError> {
        let stack = StackHeader::get_stack(&self.db, key.as_ref())?;
        match stack {
            None => Ok(vec![]),
            Some(v) => {
                Ok(v.size.to_vec())
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

struct StackHeader {
    size: [u8; 8],
    // key: &'a [u8],
}

impl StackHeader {
    fn size_i64(&self) -> i64 {
        read_int(&self.size)
    }
    fn add_1(&mut self) -> i64 {
        let mut t = read_int(&self.size);
        t += 1;
        write_int(&mut self.size, t);
        t
    }

    fn add(&mut self, v: i64) -> i64 {
        let mut t = read_int(&self.size);
        t += v;
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

    fn get_stack(db: &ckb_rocksdb::TransactionDB, key: &[u8]) -> Result<Option<Self>, RrError> {
        let v = db.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                if v.len() < 8 {
                    Err(RrError::data_error("key < 8"))
                } else {
                    Ok(Some(StackHeader::from(v.deref())))
                }
            }
        }
    }
    fn get_index(&self, db: &ckb_rocksdb::TransactionDB, key: &[u8], index: i64) -> Result<Vec<u8>, RrError> {
        let v = db.get(key)?;
        match v {
            None => Ok(vec![]),
            Some(v) => {
                if v.len() < 8 {
                    Err(RrError::data_error("key < 8"))
                } else {
                    Ok(v.to_vec())
                }
            }
        }
    }
}

impl From<i64> for StackHeader {
    fn from(value: i64) -> Self {
        let mut s = StackHeader { size: [0; 8] };
        write_int(&mut s.size, value);
        s
    }
}

impl From<&[u8]> for StackHeader {
    fn from(value: &[u8]) -> Self {
        let mut s = StackHeader { size: [0; 8] };
        s.size.copy_from_slice(value);
        s
    }
}

impl AsRef<[u8]> for StackHeader {
    fn as_ref(&self) -> &[u8] {
        &self.size
    }
}

impl Bytes for StackHeader {}
