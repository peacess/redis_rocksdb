use crate::Bytes;
use crate::RrError;

/// key+index
pub trait Stack {
	/// get the index of data
	fn index<K: Bytes>(&self, key: &K, index: i64) -> Result<Vec<u8>, RrError>;
	/// let of list, if the list do not exist return is -1
	fn len<K: Bytes>(&self, key: &K) -> Result<i64, RrError>;
	/// push a value to end, if the list do not exit, create it and push
	fn push<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i64, RrError>;
	fn pushs<K: Bytes, V: Bytes>(&mut self, key: &K, values: &[&V]) -> Result<i64, RrError>;

	/// push a value if the list exist. if list do not exist, return -1 and do nothing
	fn push_exists<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<i64, RrError>;
	/// 返回在range范围内的元素，所以start与stop可能会在list的下标之外。range是包含stop的
	/// 如果一个都没有找到，返回为len为0的Vec
	/// 0表示第一个元素
	/// -1表示倒数第一个元素
	/// -100 100表示从到数100个元素到第101个元素。如果这时list中只有3个元素，返回所有的值，因为这3个都在 range的范围之内
	fn range<K: Bytes>(&self, key: &K, start: i64, stop: i64) -> Result<Vec<Vec<u8>>, RrError>;
	/// index invalid or list is empty，return error.
	fn set<K: Bytes, V: Bytes>(&mut self, key: &K, index: i64, value: &V) -> Result<Vec<u8>, RrError>;
	/// remove the value of end
	fn pop<K: Bytes>(&self, key: &K) -> Result<Vec<u8>, RrError>;
	/// remove the value of end
	fn pops<K: Bytes>(&self, key: &K, amount: u64) -> Result<Vec<Vec<u8>>, RrError>;
	/// pop the last value to other stack
	fn poplpush<K: Bytes, V: Bytes>(&mut self, key: &K, dstkey: &K) -> Result<V, RrError>;

	/// clear the stack, return the len of stack. if the stack do not exist, return -1
	fn clear<K: Bytes>(&mut self, key: &K) -> Result<i64, RrError>;
}
