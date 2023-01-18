pub struct RedisRocksdb {
    pub(crate) db: rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: rocksdb::TransactionDB) -> Self {
        RedisRocksdb {
            db,
        }
    }
}
