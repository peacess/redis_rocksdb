use crate::{RedisSet, Stack};
use crate::rocksdb_impl::stack_impl;

pub struct RedisRocksdb {
    pub(crate) db: ckb_rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: ckb_rocksdb::TransactionDB) -> Self {
        RedisRocksdb {
            db,
        }
    }

    // pub fn get_set<T: RedisSet>(&mut self) -> T {
    //     return self;
    // }

    pub fn get_stach(&mut self) -> impl Stack {
        return self;
    }
}
