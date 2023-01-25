
use crate::{Object, RrError};

///
pub trait Heap<T> {
    /// 弹出堆，
    /// 返回值 0: field, 1: field value
    fn pop(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>,Vec<u8>)>, RrError>;
    /// 此方法与 [Object::set]功能相同
    fn push(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;
}