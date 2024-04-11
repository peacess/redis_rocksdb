use crate::{BitObject, MaxHeap, MinHeap, ObjectImp};

pub struct RedisRocksdb {
	pub(crate) db: rocksdb::TransactionDB,
}

impl RedisRocksdb {
	pub fn new(db: rocksdb::TransactionDB) -> Self {
		RedisRocksdb { db }
	}

	pub fn object() -> ObjectImp {
		return ObjectImp {};
	}

	pub fn bit_object() -> BitObject {
		return BitObject {};
	}

	pub fn max_heap() -> MaxHeap {
		return MaxHeap {};
	}

	pub fn mix_heap() -> MinHeap {
		return MinHeap {};
	}

	pub fn get_db(&self) -> &rocksdb::TransactionDB {
		&self.db
	}
}
