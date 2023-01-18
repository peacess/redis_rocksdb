use crate::{RedisHash, RedisRocksdb, RrError};

impl RedisHash for RedisRocksdb {
    fn hash_del(&mut self, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn hash_dels(&mut self, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError> {
        todo!()
    }

    fn hash_exists(&mut self, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        todo!()
    }

    fn hash_get(&mut self, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn hash_get_all(&mut self, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn hash_keys(&mut self, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn hash_len(&mut self, key: &[u8]) -> Result<Option<i64>, RrError> {
        todo!()
    }

    fn hash_mget(&mut self, key: &[u8], fields: &[u8]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        todo!()
    }

    fn hash_set(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn hash_set_not_exist(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn hash_set_exist(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn hash_vals(&mut self, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        todo!()
    }

    fn hash_remove_key(&mut self, key: &[u8]) -> Result<(), RrError> {
        todo!()
    }
}

/// 这个集合适合字段数量比较少时使用，
/// 实现，把所有的字段名存放到一个key中，这样方便于对整个字段的管理，同样也会产生一个问题，就是不要有太多的字段
/// 每个字段的key生成方式为，为key生成一个唯一的id, 这样解决kv数据库中k冲突的问题
struct SetHeader {}
