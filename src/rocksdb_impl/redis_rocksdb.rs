use crate::{ObjectImp, ObjectTransImp};

pub struct RedisRocksdb {
    pub(crate) db: rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: rocksdb::TransactionDB) -> Self {
        RedisRocksdb {
            db,
        }
    }

    pub fn kv_set() -> ObjectImp {
        return ObjectImp {};
    }

    pub fn kv_set_tr() -> ObjectTransImp {
        return ObjectTransImp {};
    }
}
