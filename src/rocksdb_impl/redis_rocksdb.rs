pub struct RedisRocksdb {
    pub(crate) db: ckb_rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: ckb_rocksdb::TransactionDB) -> Self {
        RedisRocksdb { db }
    }
}
