use rocksdb::{DBIteratorWithThreadMode, Transaction, TransactionDB};

use crate::{RrError, WrapDb};

pub struct WrapTransactionDB<'a> {
    pub db: &'a TransactionDB,
}

impl<'a> WrapDb for WrapTransactionDB<'a> {
    type Db = TransactionDB;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        Ok(self.db.get(key)?)
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), RrError> {
        Ok(self.db.put(key, value)?)
    }

    fn delete(&self, key: &[u8]) -> Result<(), RrError> {
        Ok(self.db.delete(key)?)
    }

    /// 由于[rocksdb::TransactionDB]没有[rocksdb::DB::key_may_exist]方法，所以只能取一次key value
    fn exist(&self, key: &[u8]) -> Result<bool, RrError> {
        Ok(self.db.get(key)?.is_some())
    }

    fn get_db(&self) -> &Self::Db {
        self.db
    }

    fn prefix_iterator<'b: 'c, 'c>(&'b self, prefix: &[u8]) -> DBIteratorWithThreadMode<'c, Self::Db> {
        self.db.prefix_iterator(prefix)
    }
}

pub struct WrapTransaction<'a> {
    pub db: &'a Transaction<'a, TransactionDB>,
}

impl<'a> WrapDb for WrapTransaction<'a> {
    type Db = Transaction<'a, TransactionDB>;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        Ok(self.db.get(key)?)
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), RrError> {
        Ok(self.db.put(key, value)?)
    }

    fn delete(&self, key: &[u8]) -> Result<(), RrError> {
        Ok(self.db.delete(key)?)
    }

    /// 由于[rocksdb::TransactionDB]没有[rocksdb::DB::key_may_exist]方法，所以只能取一次key value
    fn exist(&self, key: &[u8]) -> Result<bool, RrError> {
        Ok(self.db.get(key)?.is_some())
    }
    fn get_db(&self) -> &Self::Db {
        self.db
    }

    fn prefix_iterator<'b: 'c, 'c>(&'b self, prefix: &[u8]) -> DBIteratorWithThreadMode<'c, Self::Db> {
        self.db.prefix_iterator(prefix)
    }
}

pub struct WrapRocksDb<'a> {
    pub db: &'a rocksdb::DB,
}

impl<'a> WrapDb for WrapRocksDb<'a> {
    type Db = rocksdb::DB;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        Ok(self.db.get(key)?)
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), RrError> {
        Ok(self.db.put(key, value)?)
    }

    fn delete(&self, key: &[u8]) -> Result<(), RrError> {
        Ok(self.db.delete(key)?)
    }

    fn exist(&self, key: &[u8]) -> Result<bool, RrError> {
        Ok(self.db.key_may_exist(key))
    }

    fn get_db(&self) -> &Self::Db {
        self.db
    }

    fn prefix_iterator<'b: 'c, 'c>(&'b self, prefix: &[u8]) -> DBIteratorWithThreadMode<'c, Self::Db> {
        self.db.prefix_iterator(prefix)
    }
}