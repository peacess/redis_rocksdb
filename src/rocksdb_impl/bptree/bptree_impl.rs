use crate::{BPTree, LenType, RrError, WrapDb};

/// see
/// [Writing a storage engine in Rust: Writing a persistent BTree (Part 1)] (https://nimrodshn.medium.com/writing-a-storage-engine-in-rust-writing-a-persistent-btree-part-1-916b6f3e2934)
/// [A persistent copy-on-write B+Tree implementation, designed as an index for a key-value store, inspired by SQLite](https://github.com/nimrodshn/btree)
pub struct BPTreeImpl {}

impl<T: WrapDb> BPTree<T> for BPTreeImpl {
    fn set_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        todo!()
    }

    fn set_not_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        todo!()
    }

    fn set(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        todo!()
    }

    fn del_first(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        todo!()
    }

    fn del_last(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        todo!()
    }

    fn del(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn get_first(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        todo!()
    }

    fn get_last(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        todo!()
    }

    fn get(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError> {
        todo!()
    }

    fn del_key(&self, t: &T, key: &[u8]) -> Result<(), RrError> {
        todo!()
    }
}