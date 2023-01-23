use std::{mem, ptr, slice};

use rocksdb::TransactionDB;

use crate::{Object, read_int, read_int_ptr, RrError, write_int_ptr};
use crate::rocksdb_impl::make_key;

/// bst -- Binary Search Tree
/// 字段使用二叉查找树进行，增删改查管理
pub struct BitObjectBst {}

impl Object<TransactionDB> for BitObjectBst {
    fn del(&self, t: &TransactionDB, key: &[u8], field: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let mut f = BitFieldSorted::new(fv);
            f.del(field);
            t.put(&head_key, &f.data)?;
        }
        let new_key = make_key(key, field);
        t.delete(&new_key)?;

        Ok(())
    }

    fn dels(&self, t: &TransactionDB, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError> {
        let mut count = 0;
        for f in fields {
            let new_key = make_key(key, f);
            t.delete(&new_key)?;
        }
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let mut f = BitFieldSorted::new(fv);
            for field in fields {
                if f.del(field) {
                    count += 1;
                }
            }
            t.put(&head_key, &f.data)?;
        }

        Ok(count)
    }

    fn exists(&self, t: &TransactionDB, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        let new_key = make_key(key, field);
        let old = t.get(&new_key)?;
        Ok(old.is_some())
    }

    fn get(&self, t: &TransactionDB, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        let new_key = make_key(key, field);
        let v = t.get(&new_key)?;
        return Ok(v);
    }

    fn get_all(&self, t: &TransactionDB, key: &[u8]) -> Result<Option<Vec<(Vec<u8>, Vec<u8>)>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitFieldSorted::new(fv);
            let mut re = Vec::with_capacity(few_field.len());
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                let v = t.get(new_key)?;
                if let Some(v) = v {
                    re.push((field.field.to_vec(), v));
                } else {
                    re.push((field.field.to_vec(), vec![]));
                }
            }
            Ok(Some(re))
        } else {
            return Ok(None);
        }
    }

    fn keys(&self, t: &TransactionDB, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitFieldSorted::new(fv);
            let mut re = Vec::with_capacity(few_field.len());
            for field in few_field.new_field_it() {
                re.push(field.field.to_vec());
            }
            Ok(Some(re))
        } else {
            return Ok(None);
        }
    }

    fn len(&self, t: &TransactionDB, key: &[u8]) -> Result<Option<i64>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitFieldSorted::new(fv);
            Ok(Some(few_field.len() as i64))
        } else {
            return Ok(None);
        }
    }

    fn mget(&self, t: &TransactionDB, key: &[u8], fields: &[&[u8]]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        let mut values = Vec::with_capacity(fields.len());
        for f in fields {
            let new_key = make_key(key, f);
            if let Some(v) = t.get(new_key)? {
                values.push(Some(v));
            } else {
                values.push(None);
            }
        }
        Ok(values)
    }

    fn set(&self, t: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let mut few_field = BitFieldSorted::new(fv);
            few_field.set(field);
            t.put(&head_key, &few_field.data)?;
        } else {
            let mut few_field = BitFieldSorted::new(Vec::with_capacity(mem::size_of::<LenBitFieldSorted>()));
            few_field.init();
            few_field.set(field);
            t.put(&head_key, &few_field.data)?;
        }
        let new_key = make_key(key, field);
        t.put(&new_key, value)?;
        Ok(())
    }

    fn set_not_exist(&self, t: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_key(key, field);
        if let None = t.get(&new_key)? {
            t.put(new_key, value)?;

            let head_key = make_head_key(key);
            if let Some(fv) = t.get(&head_key)? {
                let mut few_field = BitFieldSorted::new(fv);
                few_field.set(field);
                t.put(&head_key, &few_field.data)?;
            } else {
                let mut few_field = BitFieldSorted::new(Vec::with_capacity(mem::size_of::<LenBitFieldSorted>()));
                few_field.init();
                few_field.set(field);
                t.put(&head_key, &few_field.data)?;
            }

            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn set_exist(&self, t: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_key(key, field);
        if let Some(_) = t.get(&new_key)? {
            t.put(new_key, value)?;
            //由于key是存在的，所以这里不用再修 head key了
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn vals(&self, t: &TransactionDB, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitFieldSorted::new(fv);
            let mut re = Vec::with_capacity(few_field.len());
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                let v = t.get(new_key)?;
                if let Some(v) = v {
                    re.push(v);
                } else {
                    re.push(vec![]);
                }
            }
            Ok(re)
        } else {
            return Ok(vec![]);
        }
    }

    fn remove_key(&self, t: &TransactionDB, key: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitFieldSorted::new(fv);
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                t.delete(new_key)?;
            }
            t.delete(&head_key)?;
        }
        return Ok(());
    }
}

///所有的field连续存入一遍连续的内存区中
/// [C++ Binary Search Tree array implementation](https://www.daniweb.com/programming/software-development/threads/466340/c-binary-search-tree-array-implementation)
/// [ArrayBinarySearchTree.java](http://faculty.washington.edu/moishe/javademos/jss2/ArrayBinarySearchTree.java)
pub(crate) struct BitFieldSorted {
    pub(crate) data: Vec<u8>,
    /// 为bst分配置的空间大小， 默认为256，增加方式 每次增加256个
    bst_capt: isize,
}

//每一个字段的byte数的类型
pub(crate) type SizeBitFieldSorted = i32;
//字段个数（len）的类型
pub(crate) type LenBitFieldSorted = i64;

impl BitFieldSorted {
    const SIZE: usize = mem::size_of::<SizeBitFieldSorted>();
    const BST_OFFSET: isize = 2 * (mem::size_of::<LenBitFieldSorted>() as isize);

    pub fn new(data: Vec<u8>) -> Self {
        BitFieldSorted { data, bst_capt: 256 }
    }

    pub fn init(&mut self) {
        if self.data.is_empty() {
            self.data.resize(mem::size_of::<LenBitFieldSorted>(), 0);
        }
    }
    /// 返回值true: 字段存在
    pub fn del(&mut self, field: &[u8]) -> bool {
        let (start, field_size) = self.find(field);
        if start >= 0 {
            let end = start + BitFieldSorted::SIZE as isize + field_size as isize;
            let p = self.data.as_ptr();
            unsafe {
                ptr::copy(p.offset(end), p.offset(start).cast_mut(), self.data.len() - end as usize);
                self.data.set_len(self.len() - field_size as usize - BitFieldSorted::SIZE);
            }
            true
        } else {
            false
        }
    }
    /// 返回值true: 字段存在
    pub fn set(&mut self, field: &[u8]) -> bool {
        let (start, _) = self.find(field);
        if start >= 0 {
            //已存在，直接返回
            true
        } else {
            //把字段加入最后
            let add = BitFieldSorted::SIZE + field.len();
            self.data.reserve(add);
            unsafe {
                let p = self.data.as_mut_ptr().offset(self.len() as isize - add as isize);
                //写入字段的bytes数量
                write_int_ptr(p, field.len() as SizeBitFieldSorted);
                //写入字段
                ptr::copy_nonoverlapping(field.as_ptr(), p.offset(BitFieldSorted::SIZE as isize), field.len());
                let len = self.len() + 1;
                //写入总的字段个数
                write_int_ptr(self.data.as_mut_ptr(), len as LenBitFieldSorted);
            }
            false
        }
    }

    pub fn len(&self) -> usize {
        let l = read_int::<LenBitFieldSorted>(&self.data);
        return l as usize;
    }

    /// 返回值 0： 开始偏移，如果没有找到为-1
    /// 返回值 1： field size
    fn find(&self, field: &[u8]) -> (isize, usize) {
        let l = self.len();
        let p = self.data.as_ptr();
        let mut offset = mem::size_of::<LenBitFieldSorted>() as isize;
        unsafe {
            for _i in 0..l {
                let field_size = read_int_ptr::<SizeBitFieldSorted>(p.offset(offset)) as usize;
                offset += BitFieldSorted::SIZE as isize;
                let f = slice::from_raw_parts(p.offset(offset), field_size);
                if f == field {
                    let start = offset - BitFieldSorted::SIZE as isize;
                    return (start, field_size);
                }
                //指向下一个field
                offset += field_size as isize;
            }
        }
        return (-1, 0);
    }

    pub(crate) fn new_field_it(&self) -> BitFieldSortedIt {
        BitFieldSortedIt::new(self)
    }
}

pub(crate) struct BitFieldSortedIt<'a> {
    data: &'a BitFieldSorted,
    len: isize,
    index: isize,
    offset: isize,
}

impl<'a> BitFieldSortedIt<'a> {
    pub fn new(d: &'a BitFieldSorted) -> Self {
        BitFieldSortedIt {
            data: d,
            len: 0,
            index: -1,
            offset: 0,
        }
    }
}

pub(crate) struct FieldItValue<'a> {
    pub(crate) field: &'a [u8],
}

impl<'a> Iterator for BitFieldSortedIt<'a> {
    type Item = FieldItValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        if self.index < 0 {
            self.len = self.data.len() as isize;
            if self.len < 1 {
                return None;
            }
            self.offset = mem::size_of::<LenBitFieldSorted>() as isize;
        }

        self.index += 1;
        let field_size = read_int_ptr::<SizeBitFieldSorted>(unsafe { self.data.data.as_ptr().offset(self.offset) });
        let it = FieldItValue {
            field: unsafe { slice::from_raw_parts(self.data.data.as_ptr().offset(self.offset + BitFieldSorted::SIZE as isize), field_size as usize) },
        };
        return Some(it);
    }
}

#[inline]
pub(crate) fn make_head_key(key: &[u8]) -> Vec<u8> {
    return make_key(key, &[]);
}


