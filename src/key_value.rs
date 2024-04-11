use crate::{Bytes, RrError};

pub trait KeyValue {
	fn get<K: Bytes, V: Bytes>(&self, key: &K) -> Result<Option<Vec<u8>>, RrError>;
	fn put<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<(), RrError>;
}
