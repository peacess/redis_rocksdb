use crate::{LenType, RrError};

/// b+ tree(binary plus tree)
pub trait BPTree<T> {
	/// 当值存在时插入，是更新值（没有使用update是为了含义更明确）
	fn set_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;
	/// 当值不存在时插入
	fn set_not_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;
	/// 如果不存在，插入，如果存在，就更新
	fn set(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError>;
	/// 取出第一个值（最小值），并删除
	/// 返回值 0: field, 1: field value
	fn del_first(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
	/// 取出最后个值（最大值），并删除
	/// 返回值 0: field, 1: field value
	fn del_last(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
	/// 删除field
	fn del(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
	/// 读取第一个值（最小值)
	fn get_first(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
	/// 读取最后一个值（最小值）
	fn get_last(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError>;
	fn get(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError>;
	fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError>;

	/// 删除指定的key，及所有字段
	fn del_key(&self, t: &T, key: &[u8]) -> Result<(), RrError>;
}
