use std::{fs, path};

use ckb_rocksdb::prelude::Open;
use ckb_rocksdb::TransactionDB;
use function_name::named;

use redis_rocksdb::{RedisList, RedisRocksdb};

fn open_db(name: &str) -> TransactionDB {
    let file_name = format!("temp/{}.db", name);
    let db_path = path::Path::new(&file_name);
    if !db_path.exists() {
        fs::create_dir_all(db_path).expect("");
    }
    ckb_rocksdb::TransactionDB::open_default(db_path).expect("")
}

#[named]
#[test]
fn test_list_lpush() {
    let db = open_db(function_name!());
    let mut redis_db = RedisRocksdb::new(db);
    let key = function_name!().as_bytes();
    let value = vec![1, 23, 6];
    let _ = redis_db.clear(&key);//先清除数据，以便测试可以反复运行
    let re = redis_db.lpush(&key, &value);
    assert_eq!(1, re.expect(""));
    assert_eq!(1, redis_db.llen(&key).expect(""));
    let get_value = redis_db.lindex(&key, 0).expect("");
    assert_eq!(value, get_value);

    let value2 = vec![2, 9];
    let re = redis_db.lpush(&key, &value2);
    assert_eq!(2, re.expect(""));
    let get_value = redis_db.lindex(&key, 0).expect("");
    assert_eq!(value2, get_value);

    let get_value = redis_db.lindex(&key, 1).expect("");
    assert_eq!(value, get_value);
}

#[named]
#[test]
fn test_list_rpush() {
    let db = open_db(function_name!());
    let mut redis_db = RedisRocksdb::new(db);
    let key = function_name!().as_bytes();
    let value = vec![1, 23, 6];
    let _ = redis_db.clear(&key);//先清除数据，以便测试可以反复运行
    let re = redis_db.rpush(&key, &value);
    assert_eq!(1, re.expect(""));
    assert_eq!(1, redis_db.llen(&key).expect(""));
    let get_value = redis_db.lindex(&key, 0).expect("");
    assert_eq!(value, get_value);

    let value2 = vec![2, 9];
    let re = redis_db.rpush(&key, &value2);
    assert_eq!(2, re.expect(""));
    let get_value = redis_db.lindex(&key, 1).expect("");
    assert_eq!(value2, get_value);

    let get_value = redis_db.lindex(&key, 0).expect("");
    assert_eq!(value, get_value);
}

#[named]
#[test]
fn test_list_lr_pop() {
    let db = open_db(function_name!());
    let mut redis_db = RedisRocksdb::new(db);
    let key = function_name!().as_bytes();
    let value = vec![1];
    let _ = redis_db.clear(&key);//先清除数据，以便测试可以反复运行

    let re = redis_db.lpop(&key);
    assert_eq!(None, re.expect(""));
    let re = redis_db.rpop(&key);
    assert_eq!(None, re.expect(""));

    let _ = redis_db.rpush(&key, &value);
    let re = redis_db.lpop(&key);
    assert_eq!(Some(value.clone()), re.expect(""));
    let re = redis_db.rpop(&key);
    assert_eq!(None, re.expect(""));
    let _ = redis_db.rpush(&key, &value);
    let re = redis_db.rpop(&key);
    assert_eq!(Some(value.clone()), re.expect(""));
    let re = redis_db.lpop(&key);
    assert_eq!(None, re.expect(""));

    let _ = redis_db.rpush(&key, &value);
    let value2 = vec![2, 9];
    let _ = redis_db.rpush(&key, &value2);
    let re = redis_db.rpop(&key);
    assert_eq!(Some(value2.clone()), re.expect(""));
    let get_value = redis_db.lindex(&key, 0).expect("");
    assert_eq!(value, get_value);
    let re = redis_db.lpop(&key);
    assert_eq!(Some(value.clone()), re.expect(""));

    let _ = redis_db.rpush(&key, &value);
    let value2 = vec![2, 9];
    let _ = redis_db.rpush(&key, &value2);
    let re = redis_db.lpop(&key);
    assert_eq!(Some(value.clone()), re.expect(""));
    let get_value = redis_db.lindex(&key, 0).expect("");
    assert_eq!(value2, get_value);
    let re = redis_db.rpop(&key);
    assert_eq!(Some(value2.clone()), re.expect(""));
}

#[named]
#[test]
fn test_list_insert_set_rem_range() {
    let db = open_db(function_name!());
    let mut redis_db = RedisRocksdb::new(db);
    let key = function_name!().as_bytes();
    let value = vec![1, 23, 6];
    let _ = redis_db.clear(&key);//先清除数据，以便测试可以反复运行

    {//list不存在时
        let re = redis_db.lset(&key, 0, &value);
        assert!(re.is_err());
        let re = redis_db.lset(&key, 1, &value);
        assert!(re.is_err());
        let re = redis_db.lrem(&key, 0, &value);
        assert_eq!(0, re.expect(""));
        let re = redis_db.lrem(&key, 1, &value);
        assert_eq!(0, re.expect(""));
        let re = redis_db.lrem(&key, -1, &value);
        assert_eq!(0, re.expect(""));


        let re = redis_db.linsert_after(&key, &value, &value);
        assert_eq!(0, re.expect(""));
        let re = redis_db.linsert_before(&key, &value, &value);
        assert_eq!(0, re.expect(""));

        let re = redis_db.lrange(&key, 0, 0);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));
        let re = redis_db.lrange(&key, 0, 1);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));
        let re = redis_db.lrange(&key, 0, -1);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));

        let re = redis_db.lrange(&key, -1, 0);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));
        let re = redis_db.lrange(&key, -1, 1);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));
        let re = redis_db.lrange(&key, -1, -1);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));
    }

    let value2 = vec![7, 8];
    {//list中有元素时
        let _ = redis_db.lpush(&key, &value);

        let re = redis_db.lset(&key, 1, &value);
        assert!(re.is_err());
        let re = redis_db.lset(&key, 0, &value2);
        assert_eq!(value, re.expect(""));
        assert_eq!(1, redis_db.llen(&key).expect(""));
        assert_eq!(vec![value2.clone()], redis_db.lrange(&key, 0, 0).expect(""));

        let re = redis_db.lset(&key, 0, &value);
        assert_eq!(value2, re.expect(""));

        //找不到给定的值，返回-1
        let re = redis_db.linsert_after(&key, &value2, &value);
        assert_eq!(-1, re.expect(""));
        let re = redis_db.linsert_before(&key, &value2, &value);
        assert_eq!(-1, re.expect(""));

        let re = redis_db.linsert_after(&key, &value, &value2);
        assert_eq!(2, re.expect(""));
        assert_eq!(2, redis_db.llen(&key).expect(""));
        let re = redis_db.linsert_after(&key, &value, &value2);
        assert_eq!(3, re.expect(""));
        assert_eq!(3, redis_db.llen(&key).expect(""));

        let re = redis_db.lrem(&key, 0, &value2);//把value2删除，只剩一个元素
        assert_eq!(2, re.expect(""));
        assert_eq!(1, redis_db.llen(&key).expect(""));

        let re = redis_db.linsert_before(&key, &value, &value2);
        assert_eq!(2, re.expect(""));
        assert_eq!(2, redis_db.llen(&key).expect(""));
        let re = redis_db.linsert_before(&key, &value, &value2);
        assert_eq!(3, re.expect(""));
        assert_eq!(3, redis_db.llen(&key).expect(""));
        let re = redis_db.linsert_before(&key, &value2, &value);
        assert_eq!(4, re.expect(""));
        assert_eq!(4, redis_db.llen(&key).expect(""));
    }
}