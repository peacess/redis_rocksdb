use crate::{Bytes, KvSet, RedisRocksdb, RrError};

impl KvSet for RedisRocksdb {
    fn kv_set_del<K: Bytes>(&mut self, key: &K, field: &K) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn kv_set_dels<K: Bytes>(&mut self, key: &K, fields: &[K]) -> Result<i64, RrError> {
        todo!()
    }

    fn kv_set_exists<K: Bytes>(&mut self, key: &K, field: &K) -> Result<bool, RrError> {
        todo!()
    }

    fn kv_set_get<K: Bytes>(&mut self, key: &K, field: &K) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn kv_set_get_all<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_keys<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_len<K: Bytes>(&mut self, key: &K) -> Result<Option<i64>, RrError> {
        todo!()
    }

    fn kv_set_mget<K: Bytes>(&mut self, key: &K, fields: &[K]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_set<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError> {
        todo!()
    }

    fn kv_set_setnx<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError> {
        todo!()
    }

    fn kv_set_vals<K: Bytes>(&mut self, key: &K) -> Result<Vec<Vec<u8>>, RrError> {
        todo!()
    }

    fn kv_set_remove_key<K: Bytes>(&mut self, key: &K) -> Result<(), RrError> {
        todo!()
    }
}

/// 这个集合适合字段数量比较少时使用，
/// 实现，把所有的字段名存放到一个key中，这样方便于对整个字段的管理，同样也会产生一个问题，就是不要有太多的字段
/// 每个字段的key生成方式为，为key生成一个唯一的id, 这样解决kv数据库中k冲突的问题
struct SetHeader {}
