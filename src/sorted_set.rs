use crate::RrError;

/// 可以存储大量的数据，在遍历数据时，性能不如redis hash
pub trait SortedSet {
	///
	fn add(&mut self, key: &[u8], score: i64, v: &[u8]) -> Result<i64, RrError>;

	/// 返回集合的数量
	/// 对应redis的zcard
	fn len(&mut self, key: &[u8]) -> Result<Option<i64>, RrError>;
}
