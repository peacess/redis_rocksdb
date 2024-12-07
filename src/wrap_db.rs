use rocksdb::{DBAccess, DBIteratorWithThreadMode};

use crate::RrError;

/// 是对db的抽象，减少在事务与不带事务时，重复的代码，如果想要更好的性能，那么可以不使用这层实现
pub trait WrapDb {
    type Db: DBAccess;
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), RrError>;
    fn delete(&self, key: &[u8]) -> Result<(), RrError>;
    /// 判断key是否存在， true存在，false不存在
    fn exist(&self, key: &[u8]) -> Result<bool, RrError>;
    /// 为了区分方法与字段，增加get
    fn get_db(&self) -> &Self::Db;
    fn prefix_iterator<'b: 'c, 'c>(&'b self, prefix: &[u8]) -> DBIteratorWithThreadMode<'c, Self::Db>;
}
