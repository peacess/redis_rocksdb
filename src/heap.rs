use crate::RrError;

/// 二叉堆（binary heap）
pub trait Heap<T> {
    /// 弹出堆中的元素（如果为最小堆，就是最小值，如果是最大堆就是最大值）
    /// 返回值 0: field, 1: field value
    fn pop(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
    ///
    fn push(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;
}