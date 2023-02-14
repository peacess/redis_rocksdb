use function_name::named;

use redis_rocksdb::{Object, RedisRocksdb, WrapDb, WrapRocksDb, WrapTransaction, WrapTransactionDB};

use crate::_redis_rocksdb::kits::{open_rocks_db, open_transaction_db};

#[named]
#[test]
fn test_object() {
    {
        let mut redis_db = RedisRocksdb::new(open_transaction_db(file!(), function_name!()));
        let wrap_db = WrapTransactionDB { db: redis_db.get_db() };
        tt_object(&wrap_db, RedisRocksdb::object());
        tt_object(&wrap_db, RedisRocksdb::bit_object());
        let trans = wrap_db.db.transaction();
        let wrap_trans = WrapTransaction { db: &trans };
        tt_object(&wrap_trans, RedisRocksdb::object());
        tt_object(&wrap_trans, RedisRocksdb::bit_object());
        let _ = trans.commit();
    }
    {
        let rocks_db = open_rocks_db(file!(), function_name!());
        let wrap_rocks_db = WrapRocksDb { db: &rocks_db };
        tt_object(&wrap_rocks_db, RedisRocksdb::object());
        tt_object(&wrap_rocks_db, RedisRocksdb::bit_object());
    }
}

fn tt_object<T: WrapDb>(wrap_db: &T, object: impl Object<T>) {
    let key = vec![0 as u8, 1, 2];
    let field = vec![6 as u8, 7, 8];
    let value = "data".to_owned();
    let _ = object.del_key(&wrap_db, &key);//删除所有内容，以便多次测试
    {//测试没有数据的情况
        let re = object.del(&wrap_db, &key, &field);
        assert_eq!((), re.expect(""));
        let values = vec![field.as_slice()];
        let re = object.dels(&wrap_db, &key, &values);
        assert_eq!(1, re.expect(""));
        let re = object.exists(&wrap_db, &key, &field);
        assert_eq!(false, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(None, re.expect(""));
        let re = object.get_all(&wrap_db, &key);
        assert_eq!(None, re.expect(""));
        let re = object.keys(&wrap_db, &key);
        assert_eq!(None, re.expect(""));
        let re = object.len(&wrap_db, &key);
        assert_eq!(None, re.expect(""));
        let re = object.mget(&wrap_db, &key, &values);
        assert_eq!(vec![None], re.expect(""));
        let re = object.set_exist(&wrap_db, &key, &field, "data".to_owned().as_bytes());
        assert_eq!(0, re.expect(""));

        let re = object.vals(&wrap_db, &key);
        assert_eq!(Vec::<Vec<u8>>::new(), re.expect(""));

        let re = object.del_key(&wrap_db, &key);
        assert_eq!((), re.expect(""));
    }

    {
        let re = object.set(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!((), re.expect(""));
        let re = object.exists(&wrap_db, &key, &field);
        assert_eq!(true, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(Some(value.as_bytes().to_vec()), re.expect(""));
        let re = object.get_all(&wrap_db, &key);
        assert_eq!(vec![(field.to_vec(), value.as_bytes().to_vec())], re.expect("").expect(""));
        let re = object.keys(&wrap_db, &key);
        assert_eq!(vec![field.clone()], re.expect("").expect(""));
        let re = object.vals(&wrap_db, &key);
        assert_eq!(vec![value.as_bytes().to_vec()], re.expect(""));

        let re = object.len(&wrap_db, &key);
        assert_eq!(Some(1), re.expect(""));
        let fields = vec![field.as_slice()];
        let re = object.mget(&wrap_db, &key, &fields);
        assert_eq!(vec![Some(value.as_bytes().to_vec())], re.expect(""));


        //测试删除
        let re = object.del(&wrap_db, &key, &field);
        assert_eq!((), re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(None, re.expect(""));
        let re = object.set(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!((), re.expect(""));
        let values = vec![field.as_slice()];
        let re = object.dels(&wrap_db, &key, &values);
        assert_eq!(1, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(None, re.expect(""));


        //测试set
        let re = object.set_exist(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!(0, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(None, re.expect(""));

        let re = object.set(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!((), re.expect(""));
        let re = object.set_exist(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!(1, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(Some(value.as_bytes().to_vec()), re.expect(""));
        let re = object.set_not_exist(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!(0, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(Some(value.as_bytes().to_vec()), re.expect(""));

        let _ = object.del(&wrap_db, &key, &field);
        let re = object.set_exist(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!(0, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(None, re.expect(""));
        let re = object.set_not_exist(&wrap_db, &key, &field, value.as_bytes());
        assert_eq!(1, re.expect(""));
        let re = object.get(&wrap_db, &key, &field);
        assert_eq!(Some(value.as_bytes().to_vec()), re.expect(""));
    }
}