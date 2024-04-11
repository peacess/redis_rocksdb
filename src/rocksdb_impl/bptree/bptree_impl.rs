use crate::rocksdb_impl::shared::make_field_key;
use crate::{BPTree, LenType, RrError, WrapDb};

/// see
/// [Writing a storage engine in Rust: Writing a persistent BTree (Part 1)] (https://nimrodshn.medium.com/writing-a-storage-engine-in-rust-writing-a-persistent-btree-part-1-916b6f3e2934)
/// [A persistent copy-on-write B+Tree implementation, designed as an index for a key-value store, inspired by SQLite](https://github.com/nimrodshn/btree)
pub struct BPTreeImpl {}

impl<T: WrapDb> BPTree<T> for BPTreeImpl {
	fn set_exist(&self, t: &T, key: &[u8], field: &[u8], _value: &[u8]) -> Result<(), RrError> {
		let field_key = make_field_key(key, field);
		if t.exist(&field_key)? {
			// let head_key = make_head_key(key);
			// let mut heap = {
			//     match t.get(&head_key)? {
			//         None => FieldHeap::new(vec![]),
			//         Some(v) => FieldHeap::new(v)
			//     }
			// };
			// let p = &mut heap as *mut _;
			// heap.init(MaxHeapCompare { heap: p });
			// heap.push(field);
			// t.put(&head_key, &heap.data)?;
		}
		// t.put(&field_key, value)?;
		Ok(())
	}

	fn set_not_exist(&self, _t: &T, _key: &[u8], _field: &[u8], _value: &[u8]) -> Result<(), RrError> {
		todo!()
	}

	fn set(&self, _t: &T, _key: &[u8], _field: &[u8], _value: &[u8]) -> Result<(), RrError> {
		todo!()
	}

	fn del_first(&self, _t: &T, _key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
		todo!()
	}

	fn del_last(&self, _t: &T, _key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
		todo!()
	}

	fn del(&self, _t: &T, _key: &[u8], _field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
		todo!()
	}

	fn get_first(&self, _t: &T, _key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
		todo!()
	}

	fn get_last(&self, _t: &T, _key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
		todo!()
	}

	fn get(&self, _t: &T, _key: &[u8], _field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
		todo!()
	}

	fn len(&self, _t: &T, _key: &[u8]) -> Result<Option<LenType>, RrError> {
		todo!()
	}

	fn del_key(&self, _t: &T, _key: &[u8]) -> Result<(), RrError> {
		todo!()
	}
}
