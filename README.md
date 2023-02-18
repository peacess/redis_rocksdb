# redis_rocksdb
rust implement structure kv(key/value) embedded database, storage by rocksdb      
Feature list  
1. redis list
2. Object, key+field
3. ObjectBit, a bit object 
4. Max/Min binary heap(zero copy)
5. B + Tree (Binary plus Tree) ...
# Sample
more details see the [test](./tests/_redis_rocksdb/test_list_impl.rs)  
Max Heap  
```rust
use rocksdb::TransactionDB;
use redis_rocksdb::{Heap, RedisRocksdb, WrapTransactionDB};

fn sample(){
    let trans_db= TransactionDB::open_default("db_name.db").expect("");
    let redis_db = RedisRocksdb::new(trans_db);
    let wrap_db = WrapTransactionDB { db: redis_db.get_db() };

    let max_heap = RedisRocksdb::max_heap();
    let key = vec![0 as u8, 1, 2];
    let field = vec![6 as u8, 7, 8];
    let value = "data".to_owned();

    let _ = max_heap.push(&wrap_db, &field, value.as_bytes());
    let _ = max_heap.pop(&wrap_db, &key);
}
```
Object 
```rust
use rocksdb::TransactionDB;
use redis_rocksdb::{Heap, Object, RedisRocksdb, WrapTransactionDB};

fn sample(){
    let trans_db= TransactionDB::open_default("db_name.db").expect("");
    let redis_db = RedisRocksdb::new(trans_db);
    let wrap_db = WrapTransactionDB { db: redis_db.get_db() };

    let object = RedisRocksdb::object();
    let key = vec![0 as u8, 1, 2];
    let field = vec![6 as u8, 7, 8];
    let value = "data".to_owned();

    let _ = object.set(&wrap_db, &field, value.as_bytes());
    let _ = object.get(&wrap_db, &key, &field);
}
```
# Install
## Window
1. install llvm
2. Environment value: set LIBCLANG_PATH=E:/lang/LLVM/lib  --- sometimes need to restart the window system for clion
## Linux


# See:

[ssdb-rocks(c++)](https://github.com/ansoda/ssdb-rocksdb)  
[ssdb](https://ssdb.io/zh_cn/)  
[rust-rocksdb, no transaction](https://github.com/rust-rocksdb/rust-rocksdb)  
[ckb-rocksdb, transaction](https://github.com/nervosnetwork/rust-rocksdb)  

注：在v0.1.0版本时使用ckb-rocksdb，在v0.2.0版本中，为了减少一次函数调用，把事务与非事务分别使用不同的接口实现，所以就没有必要再使用ckb-rocksdb  