use crate::{Bytes, KeyValue, RedisRocksdb, RrError};
use ckb_rocksdb::ops::{Get, Put};

impl RedisHash for RedisRocksdb {
    fn hget<K: Bytes, V: Bytes>(&self, key: &K) -> Result<Option<Vec<u8>>, RrError> {
        vec![]
    }
    fn hset<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<bool, RrError> {
        false
    }
    fn hlen<K: Bytes>(&mut self, key: &K) -> Result<LenType, RrError> {
        0
    }
}
