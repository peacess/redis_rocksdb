use crate::{LenType, RrError};

/// 可以存储大量的数据，在遍历数据时，性能不如redis hash
pub trait Object<T> {
    /// 删除指定的字段，并返回对应的值，如果没有返回None
    fn del(&self, t: &T, key: &[u8], field: &[u8]) -> Result<(), RrError>;
    /// 返回被成功删除字段的数量，如果字段不存在，也计算在成功删除中
    fn dels(&self, t: &T, key: &[u8], fields: &[&[u8]]) -> Result<LenType, RrError>;
    /// true: 表示存在, false: key或field不存在
    fn exists(&self, t: &T, key: &[u8], field: &[u8]) -> Result<bool, RrError>;
    fn get(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
    /// 返回所有字段与值
    fn get_all(&self, t: &T, key: &[u8]) -> Result<Option<Vec<(Vec<u8>, Vec<u8>)>>, RrError>;
    /// 返回所有的字段
    fn keys(&self, t: &T, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError>;
    /// 返回字段的数量
    fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError>;
    /// 返回值与请求顺序一样，如果字段不存在值为Ｎone
    fn mget(&self, t: &T, key: &[u8], fields: &[&[u8]]) -> Result<Vec<Option<Vec<u8>>>, RrError>;
    fn set(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;
    // fn mset<K: Bytes, V: Bytes>(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    /// 设置成功，返回 1 。 如果给定字段已经存在且没有操作被执行，返回 0
    /// 对应redis的hsetnx
    fn set_not_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;

    /// 设置成功，返回 1 。 如果给定字段已经存则执行，不存在返回 0
    fn set_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError>;
    /// 一个包含哈希表中所有值的列表。 当 key 不存在时，返回一个空表
    fn vals(&self, t: &T, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError>;

    /// 删除指定的key，及所有字段
    fn del_key(&self, t: &T, key: &[u8]) -> Result<(), RrError>;
}
