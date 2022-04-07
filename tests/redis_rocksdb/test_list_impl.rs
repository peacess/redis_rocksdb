use std::{fs, path};

use ckb_rocksdb::prelude::Open;

use redis_rocksdb::{RedisList, RedisRocksdb, to_little_endian_array};

#[test]
fn test_list_index() -> Result<(), anyhow::Error> {
    let db_path = path::Path::new("temp/test_list_index.db");
    if !db_path.exists() {
        fs::create_dir_all(db_path)?;
    }
    let db = ckb_rocksdb::TransactionDB::open_default(db_path)?;
    let mut redis_db = RedisRocksdb::new(db);
    let key = "test_index".as_bytes();
    let value = 10;
    redis_db.lpush(key, to_little_endian_array(value))?;
    let get_value = redis_db.lindex(key, 0)?;
    let len = redis_db.llen(key)?;
    assert_eq!(len, 1);
    Ok(())
}

#[test]
fn test_() {
    fn foo(a: i32, f: fn(i32) -> i32) -> i32 {
        f(a)
    }

    fn foo2<F: Fn(i32) -> i32>(a: i32, f: F) -> i32 {
        f(a)
    }
    println!("debug ");
}