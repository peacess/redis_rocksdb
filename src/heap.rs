use crate::{LenType, RrError};

/// 二叉堆（binary heap）
pub trait Heap<T> {
    /// 取出binary heap的第一个字段,并不删除, 参数及返回值参见[Heap::pop]
    fn peek(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
    /// 取出binary heap的第一个字段,并删除,(如果是min binary heap就是最小值，如果是max binary heap就是最大值)
    /// 注： 最大小最小值是以 field来比较的，并不是value的
    /// 返回值 0: field, 1: field value
    fn pop(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
    ///
    fn push(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;

    fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError>;

    /// 删除指定的key，及所有字段
    fn remove_key(&self, t: &T, key: &[u8]) -> Result<(), RrError>;
}