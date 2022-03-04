use rocksdb::Error;

use crate::{Bytes, Direction, RedisList};

pub struct RedisRocksdb {
    db: rocksdb::DB,
}

impl RedisRocksdb {
    pub fn new(db: rocksdb::DB) -> Self {
        RedisRocksdb {
            db,
        }
    }
}
