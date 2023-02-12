use function_name::named;

use redis_rocksdb::{Heap, MaxHeap, RedisRocksdb, WrapTransactionDB};

use crate::_redis_rocksdb::kits::open_transaction_db;

#[named]
#[test]
fn test_max_heap_transaction_db() {
    let db = open_transaction_db(file!(), function_name!());
    let heap = MaxHeap {};
    let wrap_db = WrapTransactionDB { db: &db };
    let key = vec![0 as u8, 1, 2];
    let f = heap.pop(&wrap_db, &key);
}