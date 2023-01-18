use rocksdb::{Transaction, TransactionDB};

use crate::RrError;

/// 可以存储大量的数据，在遍历数据时，性能不如redis hash
pub trait KvSet {
    /// 删除指定的字段，并返回对应的值，如果没有返回None
    fn kv_set_del(&self, db: &TransactionDB, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回被成功删除字段的数量，不包括的字段被忽略
    fn kv_set_dels(&self, db: &TransactionDB, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError>;
    /// true: 表示存在, false: key或field不存在
    fn kv_set_exists(&self, db: &TransactionDB, key: &[u8], field: &[u8]) -> Result<bool, RrError>;
    ///
    fn kv_set_get(&self, db: &TransactionDB, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回所有字段与值
    fn kv_set_get_all(&self, db: &TransactionDB, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回所有的字段
    fn kv_set_keys(&self, db: &TransactionDB, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回字段的数量
    fn kv_set_len(&self, db: &TransactionDB, key: &[u8]) -> Result<Option<i64>, RrError>;
    /// 返回值与请求顺序一样，如果字段不存在值为Ｎone
    fn kv_set_mget(&self, db: &TransactionDB, key: &[u8], fields: &[u8]) -> Result<Vec<Option<Vec<u8>>>, RrError>;
    /// 并且值设置成功，返回 1 。
    fn kv_set_set(&self, db: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    // fn mset<K: Bytes, V: Bytes>(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    /// 设置成功，返回 1 。 如果给定字段已经存在且没有操作被执行，返回 0
    /// 对应redis的hsetnx
    fn kv_set_set_not_exist(&self, db: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;

    /// 设置成功，返回 1 。 如果给定字段已经存则执行，不存在返回 0
    fn kv_set_set_exist(&self, db: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    /// 一个包含哈希表中所有值的列表。 当 key 不存在时，返回一个空表
    fn kv_set_vals(&self, db: &TransactionDB, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError>;

    /// 删除指定的key，及所有字段
    fn kv_set_remove_key(&self, db: &TransactionDB, key: &[u8]) -> Result<(), RrError>;
}


/// 可以存储大量的数据，在遍历数据时，性能不如redis hash
/// 带Tx后缀，表示带事务的
pub trait KvSetTr {
    /// 删除指定的字段，并返回对应的值，如果没有返回None
    fn kv_set_del(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回被成功删除字段的数量，不包括的字段被忽略
    fn kv_set_dels(&self, tr: &Transaction<TransactionDB>, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError>;
    /// true: 表示存在, false: key或field不存在
    fn kv_set_exists(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8]) -> Result<bool, RrError>;
    ///
    fn kv_set_get(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回所有字段与值
    fn kv_set_get_all(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回所有的字段
    fn kv_set_keys(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回字段的数量
    fn kv_set_len(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<i64>, RrError>;
    /// 返回值与请求顺序一样，如果字段不存在值为Ｎone
    fn kv_set_mget(&self, tr: &Transaction<TransactionDB>, key: &[u8], fields: &[u8]) -> Result<Vec<Option<Vec<u8>>>, RrError>;
    /// 并且值设置成功，返回 1 。
    fn kv_set_set(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    // fn mset<K: Bytes, V: Bytes>(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    /// 设置成功，返回 1 。 如果给定字段已经存在且没有操作被执行，返回 0
    /// 对应redis的hsetnx
    fn kv_set_set_not_exist(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;

    /// 设置成功，返回 1 。 如果给定字段已经存则执行，不存在返回 0
    fn kv_set_set_exist(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    /// 一个包含哈希表中所有值的列表。 当 key 不存在时，返回一个空表
    fn kv_set_vals(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError>;

    /// 删除指定的key，及所有字段
    fn kv_set_remove_key(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<(), RrError>;
}