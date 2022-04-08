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