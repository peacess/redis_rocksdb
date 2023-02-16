use function_name::named;
use rocksdb::DBAccess;

use redis_rocksdb::{Heap, MaxHeap, RedisRocksdb, WrapDb, WrapRocksDb, WrapTransaction, WrapTransactionDB};

use crate::_redis_rocksdb::kits::{open_rocks_db, open_transaction_db};

#[named]
#[test]
fn test_heap() {
    {
        let mut redis_db = RedisRocksdb::new(open_transaction_db(file!(), function_name!()));

        let wrap_db = WrapTransactionDB { db: redis_db.get_db() };
        tt_heap(&wrap_db, RedisRocksdb::max_heap());
        tt_heap(&wrap_db, RedisRocksdb::mix_heap());

        let trans = redis_db.get_db().transaction();
        let wrap_trans = WrapTransaction { db: &trans };
        tt_heap(&wrap_trans, RedisRocksdb::max_heap());
        tt_heap(&wrap_trans, RedisRocksdb::mix_heap());
        let _ = trans.rollback();
    }

    {
        let rocks_db = open_rocks_db(file!(), function_name!());
        let wrap_rocks_db = WrapRocksDb { db: &rocks_db };
        tt_heap(&wrap_rocks_db, RedisRocksdb::max_heap());
        tt_heap(&wrap_rocks_db, RedisRocksdb::mix_heap());
    }
}

fn tt_heap<T: WrapDb>(db: &T, heap: impl Heap<T>) {
    let key = vec![0 as u8, 1, 2];
    let field = vec![6 as u8, 7, 8];
    let value = "data".to_owned();

    let _ = heap.remove_key(db, &key);

    {
        let re = heap.peek(db, &key);
        assert_eq!(None, re.expect(""));
        let re = heap.pop(db, &key);
        assert_eq!(None, re.expect(""));
        let re = heap.len(db, &key);
        assert_eq!(None, re.expect(""));
    }
    {
        let re = heap.push(db, &key, &field, value.as_bytes());
        assert_eq!((), re.expect(""));
        let re = heap.peek(db, &key);
        assert_eq!((field.to_vec(), value.as_bytes().to_vec()), re.expect("").expect(""));
        let re = heap.len(db, &key);
        assert_eq!(Some(1), re.expect(""));
        let re = heap.pop(db, &key);
        assert_eq!((field.to_vec(), value.as_bytes().to_vec()), re.expect("").expect(""));
        let re = heap.peek(db, &key);
        assert_eq!(None, re.expect(""));
        let re = heap.pop(db, &key);
        assert_eq!(None, re.expect(""));
        let re = heap.len(db, &key);
        assert_eq!(Some(0), re.expect(""));
    }

    {
        let re = heap.push(db, &key, &field, value.as_bytes());
        assert_eq!((), re.expect(""));
        let re = heap.push(db, &key, &field, value.as_bytes());
        assert_eq!((), re.expect(""));
        let re = heap.peek(db, &key);
        assert_eq!((field.to_vec(), value.as_bytes().to_vec()), re.expect("").expect(""));
        let re = heap.len(db, &key);
        assert_eq!(Some(1), re.expect(""));
        let re = heap.pop(db, &key);
        assert_eq!((field.to_vec(), value.as_bytes().to_vec()), re.expect("").expect(""));
    }
}