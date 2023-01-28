
use crate::{Heap, RrError};
use crate::rocksdb_impl::heap::heap::{FieldHeap, MaxHeapCompare};
use crate::rocksdb_impl::shared::{make_head_key, make_key};

/// 字段名使用 max head存放
pub struct MaxHeap {}

impl Heap<rocksdb::DB> for MaxHeap {
    fn pop(&self, t: &rocksdb::DB, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        let head_key = make_head_key(key);
        let mut heap = {
            match t.get(&head_key)? {
                None => return Ok(None),
                Some(v) => FieldHeap::new(v)
            }
        };
        let p = &mut heap as *mut _;
        heap.init(MaxHeapCompare { heap: p });
        let field = match heap.pop() {
            None => return Ok(None),
            Some(f) => f
        };
        let field_key = make_key(key, &field);
        let v = {
            match t.get(field_key)? {
                None => vec![],
                Some(v) => v
            }
        };
        t.put(&head_key, &heap.data)?;
        Ok(Some((field, v)))
    }

    fn push(&self, t: &rocksdb::DB, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        let field_key = make_key(key, field);
        if !t.key_may_exist(&field_key) {
            let head_key = make_head_key(key);
            let mut heap = {
                match t.get(&head_key)? {
                    None => FieldHeap::new(vec![]),
                    Some(v) => FieldHeap::new(v)
                }
            };
            let p = &mut heap as *mut _;
            heap.init(MaxHeapCompare { heap: p });
            heap.push(field);
            t.put(&head_key, &heap.data)?;
        }
        t.put(&field_key, value)?;
        Ok(())
    }
}







