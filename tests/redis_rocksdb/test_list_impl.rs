use std::{fs, path};

use ckb_rocksdb::prelude::Open;

use redis_rocksdb::{RedisList, RedisRocksdb, to_little_endian_array};

#[test]
fn test_list_lpush() {
    let db_path = path::Path::new("temp/test_list_index.db");
    if !db_path.exists() {
        fs::create_dir_all(db_path).expect("");
    }
    let db = ckb_rocksdb::TransactionDB::open_default(db_path).expect("");
    let mut redis_db = RedisRocksdb::new(db);
    let key = "test_list_lpush".as_bytes();
    let value = vec![1, 23, 6];
    redis_db.clear(&key);//先清除数据，以便测试可以反复运行
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

#[test]
fn test_list_rpush() {
    let db_path = path::Path::new("temp/test_list_index.db");
    if !db_path.exists() {
        fs::create_dir_all(db_path).expect("");
    }
    let db = ckb_rocksdb::TransactionDB::open_default(db_path).expect("");
    let mut redis_db = RedisRocksdb::new(db);
    let key = "test_list_rpush".as_bytes();
    let value = vec![1, 23, 6];
    redis_db.clear(&key);//先清除数据，以便测试可以反复运行
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