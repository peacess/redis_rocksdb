use std::{mem, ptr, slice};
use std::cmp::Ordering;

use rocksdb::TransactionDB;

use crate::{Heap, Object, read_int, read_int_ptr, RrError, write_int, write_int_ptr};
use crate::rocksdb_impl::make_key;

/// 字段名使用 max head存放
pub struct MaxHeap {}

impl Heap<TransactionDB> for MaxHeap {
    fn pop(&self, t: &TransactionDB, key: &[u8]) -> Result<Option<(Vec<u8>, Vec<u8>)>, RrError> {
        todo!()
    }

    fn push(&self, t: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        todo!()
    }
}

///所有的field连续存入一遍连续的内存区中
/// [C++ Binary Search Tree array implementation](https://www.daniweb.com/programming/software-development/threads/466340/c-binary-search-tree-array-implementation)
/// [ArrayBinarySearchTree.java](http://faculty.washington.edu/moishe/javademos/jss2/ArrayBinarySearchTree.java)
/// [binary-search-tree(not array)](https://www.geeksforgeeks.org/binary-search-tree-set-1-search-and-insertion/?ref=lbp)
/// [binary-search-tree(not array)](https://www.javatpoint.com/binary-search-tree)
pub(crate) struct FieldMaxHeap {
    pub(crate) data: Vec<u8>,
    /// 为bst分配置的空间大小， 默认为256，增加方式 每次增加256个
    bst_capt: isize,
}

//每一个字段的byte数的类型
pub(crate) type SizeFieldMinHeap = i32;
//字段个数（len）的类型
pub(crate) type LenFieldMinHeap = i64;

pub(crate) struct FieldMeta {
    offset: isize,
}


impl FieldMaxHeap {
    const SIZE: usize = mem::size_of::<SizeFieldMinHeap>();
    const BST_OFFSET: isize = 2 * (mem::size_of::<LenFieldMinHeap>() as isize);

    pub fn new(data: Vec<u8>) -> Self {
        let mut data = data;
        let mut bst_capt = 256 as isize;
        if data.is_empty() {
            data.resize(2 * mem::size_of::<LenFieldMinHeap>() + bst_capt as usize, 0);
        } else {
            unsafe { bst_capt = read_int_ptr::<i64>(data.as_ptr().offset(mem::size_of::<LenFieldMinHeap>() as isize)) as isize; }
        };
        FieldMaxHeap { data, bst_capt }
    }

    fn make_head(&mut self) -> binary_heap_plus::BinaryHeap<FieldMeta, MaxHeapCompare<'_>> {
        let head_array = unsafe { Vec::from_raw_parts(self.data.as_mut_ptr().offset(FieldMaxHeap::BST_OFFSET as isize) as *mut FieldMeta, self.len(), self.bst_capt as usize) };
        let t = unsafe { binary_heap_plus::BinaryHeap::from_vec_cmp_raw(head_array, MaxHeapCompare { data: &self.data }, false) };
        return t;
    }

    /// 计算字段的偏移位置
    fn field_offset(&self) -> isize {
        FieldMaxHeap::BST_OFFSET + self.bst_capt
    }
    /// 返回值true: 字段存在
    pub fn pop(&mut self) -> Option<Vec<u8>> {
        let mut heap = self.make_head();
        let v = heap.pop();
        if let Some(v) = v {
            let l = self.len();
            self.set_len(l - 1);
            let start = v.offset + self.field_offset();
            let field_size = unsafe { read_int_ptr::<LenFieldMinHeap>(self.data.as_ptr().offset(start)) };
            let end = start + FieldMaxHeap::SIZE as isize + field_size as isize;
            let p = self.data.as_ptr();
            let re = self.data[start as usize + field_size as usize..end as usize].to_vec();
            unsafe {
                ptr::copy(p.offset(end), p.offset(start).cast_mut(), self.data.len() - end as usize);
                self.data.set_len(self.len() - field_size as usize - FieldMaxHeap::SIZE);
            }
            Some(re)
        } else {
            None
        }
    }
    /// 由于head结构查找很慢，所以不能插入相同的key
    pub fn push(&mut self, field: &[u8]) {
        //把字段加入最后
        let add = FieldMaxHeap::SIZE + field.len();
        self.data.reserve(add);
        unsafe {
            let p = self.data.as_mut_ptr().offset(self.len() as isize - add as isize);
            //写入字段的bytes数量
            write_int_ptr(p, field.len() as SizeFieldMinHeap);
            //写入字段
            ptr::copy_nonoverlapping(field.as_ptr(), p.offset(FieldMaxHeap::SIZE as isize), field.len());
            let len = self.len() + 1;
            //写入总的字段个数
            write_int_ptr(self.data.as_mut_ptr(), len as LenFieldMinHeap);
        }


    }

    pub fn len(&self) -> usize {
        let l = read_int::<LenFieldMinHeap>(&self.data);
        return l as usize;
    }
    pub fn set_len(&mut self, l: usize) {
        write_int::<LenFieldMinHeap>(&mut self.data, l as LenFieldMinHeap);
    }
    pub(crate) fn new_field_it(&self) -> BitFieldSortedIt {
        BitFieldSortedIt::new(self)
    }
}

struct MaxHeapCompare<'a> {
    data: &'a Vec<u8>,
}


impl<'a> compare::Compare<FieldMeta> for MaxHeapCompare<'a> {
    fn compare(&self, l: &FieldMeta, r: &FieldMeta) -> Ordering {
        todo!()
    }
}

pub(crate) struct FieldHeap<'a> {
    heap: binary_heap_plus::BinaryHeap<FieldMeta, &'a mut FieldMaxHeap>,
}


pub(crate) struct BitFieldSortedIt<'a> {
    data: &'a FieldMaxHeap,
    len: isize,
    index: isize,
    offset: isize,
}

impl<'a> BitFieldSortedIt<'a> {
    pub fn new(d: &'a FieldMaxHeap) -> Self {
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
            self.offset = mem::size_of::<LenFieldMinHeap>() as isize;
        }

        self.index += 1;
        let field_size = read_int_ptr::<SizeFieldMinHeap>(unsafe { self.data.data.as_ptr().offset(self.offset) });
        let it = FieldItValue {
            field: unsafe { slice::from_raw_parts(self.data.data.as_ptr().offset(self.offset + FieldMaxHeap::SIZE as isize), field_size as usize) },
        };
        return Some(it);
    }
}

#[inline]
pub(crate) fn make_head_key(key: &[u8]) -> Vec<u8> {
    return make_key(key, &[]);
}


