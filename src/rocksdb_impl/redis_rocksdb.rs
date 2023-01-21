use crate::rocksdb_impl::KvSetImp;
use crate::rocksdb_impl::KvSetTrImp;

pub struct RedisRocksdb {
    pub(crate) db: rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: rocksdb::TransactionDB) -> Self {
        RedisRocksdb {
            db,
        }
    }

    pub fn kv_set() -> KvSetImp {
        return KvSetImp {};
    }

    pub fn kv_set_tr() -> KvSetTrImp {
        return KvSetTrImp {};
    }
}
