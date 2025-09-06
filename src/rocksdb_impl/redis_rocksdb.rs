use crate::{BitObject, MaxHeap, MinHeap, ObjectImp};

pub struct RedisRocksdb {
    pub(crate) db: rocksdb::TransactionDB,
}

impl RedisRocksdb {
    pub fn new(db: rocksdb::TransactionDB) -> Self {
        RedisRocksdb { db }
    }

    pub fn object() -> ObjectImp {
        ObjectImp {}
    }

    pub fn bit_object() -> BitObject {
        BitObject {}
    }

    pub fn max_heap() -> MaxHeap {
        MaxHeap {}
    }

    pub fn mix_heap() -> MinHeap {
        MinHeap {}
    }

    pub fn get_db(&self) -> &rocksdb::TransactionDB {
        &self.db
    }
}
