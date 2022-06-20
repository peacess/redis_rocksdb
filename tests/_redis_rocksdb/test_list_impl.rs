use std::{fs, path};

use ckb_rocksdb::prelude::Open;
use ckb_rocksdb::TransactionDB;
use function_name::named;

use redis_rocksdb::{open, RedisList, RedisRocksdb};

fn open_db(name: &str) -> TransactionDB {
    open(format!("temp/{}.db", name)).expect("")
}

#[named]
#[test]
fn test_list_lpush() {
    let db = open_db(function_name!());
    let mut redis_db = RedisRocksdb::new(db);
    let key = function_name!().as_bytes();
    let value = vec![1, 23, 6];
    let _ = redis_db.clear(&key); //先清除数据，以便测试可以反复运行
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
    let _ = redis_db.clear(&key); //先清除数据，以便测试可以反复运行
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
    let _ = redis_db.clear(&key); //先清除数据，以便测试可以反复运行

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
    let _ = redis_db.clear(&key); //先清除数据，以便测试可以反复运行

    {
        //list不存在时
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
    let value3 = vec![9, 10];
    let value4 = vec![11];
    {
        //list中有元素时
        let _ = redis_db.lpush(&key, &value);

        let re = redis_db.lset(&key, 1, &value);
        assert!(re.is_err());
        let re = redis_db.lset(&key, 0, &value2);
        assert_eq!(value, re.expect(""));
        assert_eq!(1, redis_db.llen(&key).expect(""));
        assert_eq!(vec![value2.clone()], redis_db.lrange(&key, 0, 0).expect(""));
        assert_eq!(vec![value2.clone()], redis_db.lrange(&key, 0, 1).expect(""));
        assert_eq!(vec![value2.clone()], redis_db.lrange(&key, 0, 2).expect(""));
        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, 0, -1).expect("")
        );

        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, -1, 0).expect("")
        );
        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, -1, 1).expect("")
        );
        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, -1, 2).expect("")
        );
        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, -1, -1).expect("")
        );
        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, -2, -1).expect("")
        );

        let re = redis_db.lset(&key, 0, &value);
        assert_eq!(value2, re.expect(""));
        assert_eq!(vec![value.clone()], redis_db.lrange(&key, 0, 0).expect(""));

        //找不到给定的值，返回-1
        let re = redis_db.linsert_after(&key, &value2, &value);
        assert_eq!(-1, re.expect(""));
        let re = redis_db.linsert_before(&key, &value2, &value);
        assert_eq!(-1, re.expect(""));

        let re = redis_db.linsert_after(&key, &value, &value2);
        assert_eq!(2, re.expect(""));
        assert_eq!(2, redis_db.llen(&key).expect(""));
        let re = redis_db.linsert_after(&key, &value, &value3);
        assert_eq!(3, re.expect(""));
        assert_eq!(3, redis_db.llen(&key).expect(""));

        assert_eq!(
            vec![value.clone(), value3.clone(), value2.clone()],
            redis_db.lrange(&key, 0, -1).expect("")
        );
        let re = redis_db.lrange(&key, 0, 0).expect("");
        assert_eq!(vec![value.clone()], re);
        assert_eq!(
            vec![value2.clone()],
            redis_db.lrange(&key, -1, -1).expect("")
        );
        assert_eq!(vec![value3.clone()], redis_db.lrange(&key, 1, 1).expect(""));
        assert_eq!(
            vec![value3.clone(), value2.clone()],
            redis_db.lrange(&key, -2, -1).expect("")
        );
        assert_eq!(
            vec![value.clone(), value3.clone(), value2.clone()],
            redis_db.lrange(&key, -3, -1).expect("")
        );
        assert_eq!(
            vec![value.clone(), value3.clone(), value2.clone()],
            redis_db.lrange(&key, -4, -1).expect("")
        );

        let re = redis_db.lrem(&key, 0, &value2); //把value2删除，只剩一个元素
        assert_eq!(1, re.expect(""));
        assert_eq!(2, redis_db.llen(&key).expect(""));

        {
            //测试删除连续的值
            let _ = redis_db.linsert_before(&key, &value, &value2);
            let _ = redis_db.linsert_before(&key, &value, &value2);
            let _ = redis_db.linsert_before(&key, &value, &value2);
            let re = redis_db.lrem(&key, 0, &value2); //把value2删除，只剩一个元素
            assert_eq!(3, re.expect(""));
            assert_eq!(2, redis_db.llen(&key).expect(""));
        }

        let re = redis_db.linsert_before(&key, &value, &value2);
        assert_eq!(3, re.expect(""));
        assert_eq!(3, redis_db.llen(&key).expect(""));
        let re = redis_db.linsert_before(&key, &value, &value3);
        assert_eq!(4, re.expect(""));
        assert_eq!(4, redis_db.llen(&key).expect(""));
        let re = redis_db.linsert_before(&key, &value3, &value4);
        assert_eq!(5, re.expect(""));
        assert_eq!(5, redis_db.llen(&key).expect(""));

        //value2,value4,value3,value,value3
        assert_eq!(vec![value2.clone()], redis_db.lrange(&key, 0, 0).expect(""));
        assert_eq!(vec![value4.clone()], redis_db.lrange(&key, 1, 1).expect(""));
        assert_eq!(vec![value3.clone()], redis_db.lrange(&key, 2, 2).expect(""));
        assert_eq!(vec![value.clone()], redis_db.lrange(&key, 3, 3).expect(""));
        assert_eq!(vec![value3.clone()], redis_db.lrange(&key, 4, 4).expect(""));

        assert_eq!(
            vec![value.clone(), value3.clone()],
            redis_db.lrange(&key, -2, -1).expect("")
        );
        assert_eq!(
            vec![value3.clone(), value.clone(), value3.clone()],
            redis_db.lrange(&key, -3, -1).expect("")
        );
        assert_eq!(
            vec![
                value4.clone(),
                value3.clone(),
                value.clone(),
                value3.clone()
            ],
            redis_db.lrange(&key, -4, -1).expect("")
        );
    }
}
