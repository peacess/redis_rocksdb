use crate::{Bytes, Direction, Error, MetaKey, RedisList, RedisRocksdb};

/// [see] (https://xindoo.blog.csdn.net/article/details/109150975)
/// ssdb没有实现list，只实现了queue
///
/// redis中的list使用quicklist与ziplist实现
impl RedisList for RedisRocksdb {
    fn blpop<K: Bytes, V: Bytes>(key: K, timeout: i64) -> Result<V, Error> {
        todo!()
    }

    fn brpop<K: Bytes, V: Bytes>(key: K, timeout: i64) -> Result<V, Error> {
        todo!()
    }

    fn brpoplpush<K: Bytes, V: Bytes>(srckey: K, dstkey: K, timeout: i64) -> Result<V, Error> {
        todo!()
    }

    fn lindex<K: Bytes, V: Bytes>(key: K, index: i32) -> Result<V, Error> {
        todo!()
    }

    fn linsert_before<K: Bytes, P: Bytes, V: Bytes>(key: K, pivot: P, value: V) -> Result<(), Error> {
        todo!()
    }

    fn linsert_after<K: Bytes, P: Bytes, V: Bytes>(key: K, pivot: P, value: V) -> Result<(), Error> {
        todo!()
    }

    fn llen<K: Bytes>(key: K) -> Result<i32, Error> {
        todo!()
    }

    fn lmove<K: Bytes, V: Bytes>(srckey: K, dstkey: K, src_dir: Direction, dst_dir: Direction) -> Result<V, Error> {
        todo!()
    }

    fn lmpop<K: Bytes>(numkeys: i32, key: K, dir: Direction, count: i32) {
        todo!()
    }

    fn lpop<K: Bytes, V: Bytes>(key: K) -> Result<V, Error> {
        todo!()
    }

    fn lpush<K: Bytes, V: Bytes>(key: K, value: V) -> Result<(), Error> {
        todo!()
    }

    fn lpush_exists<K: Bytes, V: Bytes>(key: K, value: V) -> Result<(), Error> {
        todo!()
    }

    fn lrange<K: Bytes, V: Bytes>(key: K, start: i32, stop: i32) -> Result<Vec<V>, Error> {
        todo!()
    }

    fn lrem<K: Bytes, V: Bytes>(key: K, count: i32, value: V) -> Result<V, Error> {
        todo!()
    }

    fn ltrim<K: Bytes>(key: K, start: i32, stop: i32) -> Result<i32, Error> {
        todo!()
    }

    fn lset<K: Bytes, V: Bytes>(key: K, index: i32, value: V) {
        todo!()
    }

    fn rpop<K: Bytes>(key: K, count: Option<i32>) {
        todo!()
    }

    fn rpoplpush<K: Bytes, V: Bytes>(key: K, dstkey: K) -> Result<V, Error> {
        todo!()
    }

    fn rpush<K: Bytes, V: Bytes>(key: K, value: V) -> Result<(), Error> {
        todo!()
    }

    fn rpush_exists<K: Bytes, V: Bytes>(key: K, value: V) -> Result<(), Error> {
        todo!()
    }
}
