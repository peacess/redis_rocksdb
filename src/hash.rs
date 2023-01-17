use crate::{Bytes, RrError};

/// 这个对应redis中的hash, 字段数据量建议在2048个以内，在遍历数据时，性能比KvSet好
pub trait RedisHash {
    /// 删除指定的字段，并返回对应的值，如果没有返回None
    fn hash_del<K: Bytes>(&mut self, key: &K, field: &K) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回被成功删除字段的数量，不包括的字段被忽略
    fn hash_dels<K: Bytes>(&mut self, key: &K, fields: &[K]) -> Result<i64, RrError>;
    /// true: 表示存在, false: key或field不存在
    fn hash_exists<K: Bytes>(&mut self, key: &K, field: &K) -> Result<bool, RrError>;
    ///
    fn hash_get<K: Bytes>(&mut self, key: &K, field: &K) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回所有字段与值
    fn hash_get_all<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回所有的字段
    fn hash_keys<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回字段的数量
    fn hash_len<K: Bytes>(&mut self, key: &K) -> Result<Option<i64>, RrError>;
    /// 返回值与请求顺序一样，如果字段不存在值为Ｎone
    fn hash_mget<K: Bytes>(&mut self, key: &K, fields: &[K]) -> Result<Vec<Option<Vec<u8>>>, RrError>;
    /// 如果字段是哈希表中的一个新建字段，并且值设置成功，返回 1 。 如果哈希表中域字段已经存在且旧值已被新值覆盖，返回 0
    fn hash_set<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;
    // fn mset<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;
    /// 设置成功，返回 1 。 如果给定字段已经存在且没有操作被执行，返回 0
    /// 对应redis的hsetnx
    fn hash_set_not_exist<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;

    /// 设置成功，返回 1 。 如果给定字段已经存则执行，不存在返回 0
    fn hash_set_exist<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;
    /// 一个包含哈希表中所有值的列表。 当 key 不存在时，返回一个空表
    fn hash_vals<K: Bytes>(&mut self, key: &K) -> Result<Vec<Vec<u8>>, RrError>;

    /// 删除指定的key，及所有字段(这个不是redis的接口)
    fn hash_remove_key<K: Bytes>(&mut self, key: &K) -> Result<(), RrError>;
}