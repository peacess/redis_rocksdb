use crate::{Bytes, Direction};
use crate::Error;

pub trait RedisList {
    fn blpop<K: Bytes, V: Bytes>(&mut self, key: K, timeout: i64) -> Result<V, Error>;
    fn brpop<K: Bytes, V: Bytes>(&mut self, key: K, timeout: i64) -> Result<V, Error>;
    fn brpoplpush<K: Bytes, V: Bytes>(&mut self, srckey: K, dstkey: K, timeout: i64) -> Result<V, Error>;
    fn lindex<K: Bytes>(&self, key: K, index: i32) -> Result<Vec<u8>, Error>;

    /// 如果命令执行成功，返回插入操作完成之后，列表的长度。
    /// 如果没有找到指定元素 ，返回 -1 。
    /// 如果 key 不存在或为空列表，返回 0
    fn linsert_before<K: Bytes, P: Bytes, V: Bytes>(&mut self, key: K, pivot: P, value: V) -> Result<i32, Error>;
    /// 如果命令执行成功，返回插入操作完成之后，列表的长度。
    /// 如果没有找到指定元素 ，返回 -1 。
    /// 如果 key 不存在或为空列表，返回 0
    fn linsert_after<K: Bytes, P: Bytes, V: Bytes>(&mut self, key: K, pivot: P, value: V) -> Result<i32, Error>;

    // 返回值为-1表示还没有这个list
    fn llen<K: Bytes>(&self, key: K) -> Result<i32, Error>;

    fn lpop<K: Bytes>(&mut self, key: K) -> Result<Vec<u8>, Error>;
    /// 返回len of list
    fn lpush<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<i32, Error>;

    /// 返回len of list，如果list不存在返回值为 0
    fn lpush_exists<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<i32, Error>;

    fn lrange<K: Bytes, V: Bytes>(&self, key: K, start: i32, stop: i32) -> Result<Vec<V>, Error>;
    /// COUNT 的值可以是以下几种：
    /// count > 0 : 从表头开始向表尾搜索，移除与 VALUE 相等的元素，数量为 COUNT。
    /// count < 0 : 从表尾开始向表头搜索，移除与 VALUE 相等的元素，数量为 COUNT 的绝对值。
    /// count = 0 : 移除表中所有与 VALUE 相等的值
    fn lrem<K: Bytes, V: Bytes>(&mut self, key: K, count: i32, value: V) -> Result<V, Error>;
    /// 保留指定区间内的元素，不在指定区间之内的元素都将被删除, 反回删除的元素数量
    fn ltrim<K: Bytes>(&mut self, key: K, start: i32, stop: i32) -> Result<i32, Error>;

    fn lset<K: Bytes, V: Bytes>(&mut self, key: K, index: i32, value: V);
    /// 移除列表的最后一个元素
    fn rpop<K: Bytes>(&mut self, key: K) -> Result<Vec<u8>, Error>;
    /// 移除列表的最后一个元素，并将该元素添加到另一个列表并返回
    fn rpoplpush<K: Bytes, V: Bytes>(&mut self, key: K, dstkey: K) -> Result<V, Error>;
    /// 添加到列表尾部
    fn rpush<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<i32, Error>;
    /// 为已经存在的列表添加值， 添加到尾部
    fn rpush_exists<K: Bytes, V: Bytes>(&mut self, key: K, value: V) -> Result<i32, Error>;
}