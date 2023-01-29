# redis_rocksdb
rust implement structure kv(key/value) embedded database, storage by rocksdb      
Feature list  
1. redis list
2. Object, key+field
3. ObjectBit, a bit object 
4. Max/Min binary heap(zero copy)
# Sample
see the [test](./tests/_redis_rocksdb/test_list_impl.rs)
```rust

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