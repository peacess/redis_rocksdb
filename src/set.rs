use crate::{Bytes, RrError};

pub trait RedisSet {

    /// 删除指定的字段，并返回对应的值，如果没有返回None
    fn h_del<K: Bytes>(&mut self, key: &K, field: &K) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回被成功删除字段的数量，不包括的字段被忽略
    fn h_dels<K: Bytes>(&mut self, key: &K, fields: &[K]) -> Result<i64, RrError>;
    /// true: 表示存在, false: key或field不存在
    fn h_exists<K: Bytes>(&mut self, key: &K, field: &K) -> Result<bool, RrError>;
    ///
    fn h_get<K: Bytes>(&mut self, key: &K, field: &K) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回所有字段与值
    fn h_get_all<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回所有的字段
    fn h_keys<K: Bytes>(&mut self, key: &K) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回字段的数量
    fn h_len<K: Bytes>(&mut self, key: &K) -> Result<Option<i64>, RrError>;
    /// 返回值与请求顺序一样，如果字段不存在值为Ｎone
    fn h_mget<K: Bytes>(&mut self, key: &K, fields: &[K]) -> Result<Vec<Option<Vec<u8>>>, RrError>;
    /// 如果字段是哈希表中的一个新建字段，并且值设置成功，返回 1 。 如果哈希表中域字段已经存在且旧值已被新值覆盖，返回 0
    fn h_set<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;
    // fn mset<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;
    /// 设置成功，返回 1 。 如果给定字段已经存在且没有操作被执行，返回 0
    fn h_setnx<K: Bytes, V: Bytes>(&mut self, key: &K, field: &K, value: &V) -> Result<i32, RrError>;
    /// 一个包含哈希表中所有值的列表。 当 key 不存在时，返回一个空表
    fn h_vals<K: Bytes>(&mut self, key: &K) -> Result<Vec<Vec<u8>>, RrError>;

    /// 删除指定的key(这个不是redis的接口)
    fn h_remove_key<K: Bytes>(&mut self, key: &K) -> Result<(), RrError>;
}