use crate::{Heap, LenType, RrError, WrapDb};
use crate::rocksdb_impl::heap::heap::{FieldHeap, MinHeapCompare};
use crate::rocksdb_impl::shared::{make_field_key, make_head_key};

/// 字段名使用 min binary head存放
pub struct MinHeap {}

impl<T: WrapDb> Heap<T> for MinHeap {
    fn peek(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        let head_key = make_head_key(key);
        let mut heap = {
            match t.get(&head_key)? {
                None => return Ok(None),
                Some(v) => FieldHeap::new(v)
            }
        };
        let p = &mut heap as *mut _;
        heap.init(MinHeapCompare { heap: p });
        let field = match heap.peek() {
            None => return Ok(None),
            Some(f) => f
        };
        let field_key = make_field_key(key, &field);
        let v = {
            match t.get(&field_key)? {
                None => vec![],
                Some(v) => v
            }
        };
        Ok(Some((field, v)))
    }

    fn pop(&self, t: &T, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        let head_key = make_head_key(key);
        let mut heap = {
            match t.get(&head_key)? {
                None => return Ok(None),
                Some(v) => FieldHeap::new(v)
            }
        };
        let p = &mut heap as *mut _;
        heap.init(MinHeapCompare { heap: p });
        let field = match heap.pop() {
            None => return Ok(None),
            Some(f) => f
        };
        let field_key = make_field_key(key, &field);
        let v = {
            match t.get(&field_key)? {
                None => vec![],
                Some(v) => v
            }
        };
        t.put(&head_key, &heap.data)?;
        t.delete(&field_key)?;
        Ok(Some((field, v)))
    }

    fn push(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        let field_key = make_field_key(key, field);
        if !t.exist(&field_key)? {
            let head_key = make_head_key(key);
            let mut heap = {
                match t.get(&head_key)? {
                    None => FieldHeap::new(vec![]),
                    Some(v) => FieldHeap::new(v)
                }
            };
            let p = &mut heap as *mut _;
            heap.init(MinHeapCompare { heap: p });
            heap.push(field);
            t.put(&head_key, &heap.data)?;
        }
        t.put(&field_key, value)?;
        Ok(())
    }

    fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError> {
        let head_key = make_head_key(key);
        let heap = match t.get(&head_key)? {
            None => return Ok(None),
            Some(v) => FieldHeap::<MinHeapCompare>::new(v)
        };
        Ok(Some(heap.len() as LenType))
    }

    fn remove_key(&self, t: &T, key: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        let mut heap = {
            match t.get(&head_key)? {
                None => return Ok(()),
                Some(v) => FieldHeap::new(v)
            }
        };
        let p = &mut heap as *mut _;
        heap.init(MinHeapCompare { heap: p });
        loop {
            let field = match heap.pop() {
                None => break,
                Some(f) => f
            };
            let field_key = make_field_key(key, &field);
            t.delete(&field_key)?;
        }
        t.delete(&head_key)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_min_heap_transactiondb() {
        // let heap = MinHeap {};
        // let key = vec![0 as u8, 1, 2];
        // let f = heap.pop(&key);
    }
}






