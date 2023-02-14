use function_name::named;
use rocksdb::DBAccess;

use redis_rocksdb::{Heap, MaxHeap, RedisRocksdb, WrapDb, WrapRocksDb, WrapTransaction, WrapTransactionDB};

use crate::_redis_rocksdb::kits::{open_rocks_db, open_transaction_db};

#[named]
#[test]
fn test_heap() {
    let db = open_transaction_db(file!(), function_name!());
    let wrap_db = WrapTransactionDB { db: &db };
    {
        // let heaps: Vec<Box<dyn Heap<WrapTransactionDB>>> = vec![Box::new(RedisRocksdb::max_heap()), Box::new(RedisRocksdb::mix_heap())];
        // for heap in heaps {
        //     tt_heap(&wrap_db, heap);
        // }

        tt_heap(&wrap_db, Box::new(RedisRocksdb::max_heap()));
    }

    // {
    //     let heaps: Vec<Box<dyn Heap<WrapTransaction>>> = vec![Box::new(RedisRocksdb::max_heap()), Box::new(RedisRocksdb::mix_heap())];
    //     let trans = wrap_db.db.transaction();
    //
    //     let wrap_trans = WrapTransaction { db: &trans };
    //     for heap in heaps {
    //         tt_heap(&wrap_trans, heap);
    //     }
    //     let _ = trans.rollback();
    // }
    //
    // drop(db);//关闭它，重新收非事务方式打开
    // {
    //     let rocks_db = open_rocks_db(file!(), function_name!());
    //     let heaps: Vec<Box<dyn Heap<WrapRocksDb>>> = vec![Box::new(RedisRocksdb::max_heap()), Box::new(RedisRocksdb::mix_heap())];
    //     let wrap_rocks_db = WrapRocksDb { db: &rocks_db };
    //     for heap in heaps {
    //         tt_heap(&wrap_rocks_db, heap);
    //     }
    // }
}

fn tt_heap<T: WrapDb>(db: &T, heap: Box<dyn Heap<T>>) {
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
    }

    {
        let re = heap.peek(db, &key);
        assert_eq!(None, re.expect(""));
        let re = heap.pop(db, &key);
        assert_eq!(None, re.expect(""));
        let re = heap.len(db, &key);
        assert_eq!(None, re.expect(""));
    }
}