use core::ptr;
use std::env::var;
use std::iter::zip;
use std::net::SocketAddr;
use std::ops::Index;
use std::ptr::NonNull;
use std::vec::IntoIter;

use ckb_rocksdb::{Transaction, TransactionDB};
use ckb_rocksdb::prelude::Get;

use crate::{BYTES_LEN_TYPE, EndianScalar, Error, LenType, read_int, read_len_type, write_int};

///
/// ```rust
/// use redis_rocksdb::{LenType, MetaKey};
///
/// struct ZipList{
///     len: LenType,
///     values: [ZipListNode],
/// }
///
/// struct ZipListNode{
///     left_size_node: u16,
///     value: [u8],
///     right_size_node: u16,
/// }
///
/// ```

struct ZipListNode<'a>(&'a [u8],usize,usize);

type LenNode = u16;

impl<'a> ZipListNode<'a> {
    const LEN_NODE: usize = core::mem::size_of::<LenNode>();
    const OFFSET_VALUE: usize = ZipListNode::LEN_NODE;

    fn from_start(bytes: &'a [u8]) -> Option<Self> {
        let len_node = ZipListNode::read_bytes_of_value(bytes) + ZipListNode::LEN_NODE * 2;
        if len_node > bytes.len() {
            None
        } else {
            Some(Self(bytes,0, len_node))
        }
    }

    fn from_end(bytes: &'a [u8]) -> Option<Self> {
        let len_node = ZipListNode::read_bytes_of_value(&bytes[bytes.len() - ZipListNode::LEN_NODE..]) + ZipListNode::LEN_NODE * 2;
        if len_node > bytes.len() {
            None
        } else {
            Some(Self(bytes, bytes.len() - len_node, len_node))
        }
    }

    fn read_value(bytes: &'a [u8], offset: usize) -> &'a [u8]{
        let offset_value = offset + ZipListNode::read_bytes_of_value(&bytes[offset as usize..]) + ZipListNode::LEN_NODE ;
        &bytes[offset + ZipListNode::LEN_NODE..offset_value]
    }

    fn count_bytes(value: &[u8]) -> usize {
        value.len() + ZipListNode::LEN_NODE * 2
    }

    fn read_bytes_of_value(node: &[u8]) -> usize {
        read_int::<LenNode>(node) as usize
    }

    #[inline]
    fn write_value(value: &[u8], p: *mut u8) {
        let x_le = (value.len() as LenNode).to_little_endian();
        unsafe {
            ptr::copy_nonoverlapping(
                &x_le as *const LenNode as *const u8,
                p,
                ZipListNode::LEN_NODE,
            );
            ptr::copy_nonoverlapping(
                value.as_ptr(),
                p.offset(ZipListNode::LEN_NODE as isize),
                value.len(),
            );
            ptr::copy_nonoverlapping(
                &x_le as *const LenNode as *const u8,
                p.offset(ZipListNode::LEN_NODE as isize + value.len() as isize),
                ZipListNode::LEN_NODE,
            );
        }
    }

    fn bytes_of_node(&self) -> usize {
        self.2
    }

    fn bytes_of_value(&self) -> usize {
        self.2 - ZipListNode::LEN_NODE * 2
    }

    fn offset(&self) -> usize {
        self.1
    }

    fn value(&'a self) -> &'a [u8] {
        &self.0[self.1..self.1 + self.2]
    }
}

impl AsRef<[u8]> for ZipListNode<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}


pub(crate) struct ZipList(Vec<u8>);

impl From<Vec<u8>> for ZipList {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes;
        if bytes.len() < ZipList::LEN_INIT {
            bytes.resize(ZipList::LEN_INIT, 0);
        }
        ZipList(bytes)
    }
}

impl AsRef<[u8]> for ZipList {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ZipList {
    const LEN_INIT: usize = 32;
    const OFFSET_VALUE: usize = BYTES_LEN_TYPE;
    pub fn new() -> Self {
        ZipList(Vec::from([0; ZipList::LEN_INIT]))
    }


    pub(crate) fn get(tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<ZipList>, Error> {
        let v = tr.get(key)?;
        match v {
            None => Ok(None),
            Some(v) => {
                Ok(Some(ZipList::from(v.to_vec())))
            }
        }
    }

    pub fn len(&self) -> LenType {
        read_int(self.0.as_slice())
    }

    pub fn set_len(&mut self, len: LenType) {
        write_int(self.0.as_mut_slice(), len)
    }

    fn get_offset_index(&self, index: usize) -> Option<usize> {
        if index < self.len() as usize {
            let mut offset = ZipList::OFFSET_VALUE;
            for _ in 0..index {
                let len_value = ZipListNode::read_bytes_of_value(&self.0[offset..]);
                offset += len_value + ZipListNode::LEN_NODE * 2;
            }
            Some(offset)
        } else {
            None
        }
    }

    pub fn pop_left(&mut self) -> Vec<u8> {
        let len = self.len() - 1;
        self.set_len(len);

        let offset = ZipList::OFFSET_VALUE;

        let node = ZipListNode::from_start(&self.0[offset..]);
        if node.is_none() {
            return vec![];
        }
        let node = node.expect("");
        let pop_value = node.0.clone().to_vec();

        let size_node = node.bytes_of_node();
        unsafe {
            let p = self.0[offset..].as_mut_ptr();
            ptr::copy(p.offset(size_node as isize), p, size_node);
        }
        self.0.truncate(self.0.len() - size_node);

        pop_value
    }

    pub fn push_left(&mut self, value: &[u8]) {
        self.insert_left(0, value);
    }

    pub fn pop_right(&mut self) -> Vec<u8> {
        let len = self.len() - 1;
        self.set_len(len);

        let right = ZipListNode::from_end(self.0.as_slice());
        if right.is_none() {
            return vec![];
        }
        let right = right.expect("");
        let pop_value = right.0.clone().to_vec();
        let offset = self.0.len() - right.bytes_of_node();
        self.0.truncate(offset);
        pop_value
    }

    pub fn push_right(&mut self, value: &[u8]) {
        let s = self.len() + 1;
        self.set_len(s);

        let old_len = self.0.len();
        let add_len = BYTES_LEN_TYPE + value.len();

        // fn resize will set the default value, so replace with  reserve and set_len
        self.0.reserve(add_len);
        unsafe { self.0.set_len(old_len + add_len); }
        unsafe { ZipListNode::write_value(value, self.0.as_mut_ptr().offset(old_len as isize)); }
    }

    pub fn insert_left(&mut self, index: i32, value: &[u8]) -> bool {
        let offset = self.get_offset_index(index as usize);
        if let Some(offset) = offset {
            let old_bytes = self.0.len();
            let add_bytes = ZipListNode::count_bytes(value);
            let s = self.len() + 1;
            self.set_len(s);
            self.0.reserve(add_bytes);
            unsafe {
                self.0.set_len(old_bytes + add_bytes);
                let p = self.0.as_mut_ptr().offset(offset as isize);
                ptr::copy(p, p.offset(add_bytes as isize), old_bytes);
                ZipListNode::write_value(value, self.0.as_mut_ptr().offset(offset as isize));
            }
            true
        } else {
            false
        }
    }

    pub fn insert_right(&mut self, index: i32, value: &[u8]) -> bool {
        return self.insert_left(index + 1, value);
    }

    /// 没有找到pivot 返回None
    /// 找到并成功插入，返回插入后的 offset
    pub fn insert_value_left(&mut self, pivot: &[u8], value: &[u8]) -> Option<i32> {
        let mut index = 0;
        let mut it = ZipListIter::new(self);
        let find_value = it.find(|it| {
            index += 1;
            return pivot.eq(it.value());
        });
        if find_value.is_none() {
            return None;
        } else {
            it.prev();
            index -= 1;
            let add_len = ZipListNode::count_bytes(value);
            let offset = it.offset() as isize;
            let old_bytes_len = self.0.len();
            unsafe {
                self.0.reserve(add_len);
                self.0.set_len(self.0.len() + add_len);
                let p = self.0.as_mut_ptr().offset(offset as isize);
                ptr::copy(p, p.offset(add_len as isize), old_bytes_len - offset as usize);
            }
            unsafe { ZipListNode::write_value(value, self.0.as_mut_ptr().offset(offset as isize)) }
            self.set_len(self.len() + 1);
        }
        Some(index)
    }

    /// 没有找到pivot 返回None
    /// 找到并成功插入，返回插入后的 offset
    pub fn insert_value_right(&mut self, pivot: &[u8], value: &[u8]) -> Option<i32> {
        let mut index = 0;
        let mut it = ZipListIter::new(self);
        let find_value = it.find(|it| {
            index += 1;
            return pivot.eq(it.value());
        });
        if find_value.is_none() {
            return None;
        } else {
            // it.prev(); //当前就在右边
            index -= 1;
            let add_len = ZipListNode::count_bytes(value);
            let offset = it.offset() as isize;
            let old_bytes_len = self.0.len();
            unsafe {
                self.0.reserve(add_len);
                self.0.set_len(self.0.len() + add_len);
                let p = self.0.as_mut_ptr().offset(offset as isize);
                ptr::copy(p, p.offset(add_len as isize), old_bytes_len - offset as usize);
            }
            unsafe { ZipListNode::write_value(value, self.0.as_mut_ptr().offset(offset as isize)) }
            self.set_len(self.len() + 1);
        }
        Some(index)
    }

    //返回原来的值
    pub fn set(&mut self, index: i32, value: &[u8]) -> Option<Vec<u8>> {
        // todo if the index > value_len
        let offset = self.get_offset_index(index as usize);
        if let Some(offset) = offset {
            let node = ZipListNode::from_start(&self.0[offset..]);
            if node.is_none() {
                return None;
            }
            let node = node.expect("");
            let old_value = node.value().to_vec();
            let old_bytes_value = node.bytes_of_value();

            let p = unsafe{ self.0.as_mut_ptr().offset(offset as isize) };
            //这里一定要使用isize,因为可能为负数
            let diff: isize = value.len() as isize - (old_bytes_value as isize);
            if diff == 0 {
                //这种情况下 不需要移动任何数据，因为大小是一样的
            } else if diff > 0 {
                unsafe {
                    self.0.reserve(diff as usize);
                    self.0.set_len(self.0.len() + diff as usize);
                    ptr::copy(p, p.offset(diff as isize), self.0.len() - offset - diff as usize);
                }
            } else if diff < 0 {
                unsafe {
                    ptr::copy(p, p.offset(diff), self.0.len() - offset);
                }
                self.0.truncate(self.0.len() - diff as usize);
            }

            ZipListNode::write_value(value, p);
            Some(old_value)
        } else {
            return None;
        }
    }

    pub fn rem(&mut self, count: i32, value: &[u8]) -> LenType {
        let mut done: LenType = 0;
        let mut removes = Vec::<(isize, isize)>::new();
        if count > 0 {
            let mut it = ZipListIter::new(self);
            loop {
                if let Some(node) = it.next(){
                    if value.eq(node.value()) {
                        let pre_offset = it.prev_offset();
                        if let Some(offset) = pre_offset {
                            removes.push((offset as isize, (it.offset()) as isize));
                            if removes.len() >= count as usize {
                                break;
                            }
                        } else {
                            log::error!("inner error");
                        }
                    }
                }else{
                    break
                }
            }
        } else if count < 0 {
            let mut it = ZipListIter::new(self);
            it.start_cur = it.len as usize;
            loop {
                if let Some(node) = it.next_back() {
                    if value.eq(node.value()) {
                        let next_offset = it.next_offset();
                        if let Some(offset) = next_offset {
                            removes.push((it.offset() as isize, (offset) as isize));
                            if removes.len() >= count.abs() as usize {
                                break;
                            }
                        } else {
                            log::error!("inner error");
                        }
                    }
                }else{
                    break;
                }
            }
        } else {
            let mut it = ZipListIter::new(self);
            loop  {
                if let Some(node) = it.next() {
                    if value.eq(node.value()) {
                        let pre_offset = it.prev_offset();
                        if let Some(offset) = pre_offset {
                            removes.push((offset as isize, (it.offset()) as isize));
                        } else {
                            log::error!("inner error");
                        }
                    }
                }else{
                    break
                }
            }
        }

        let will_remove = removes.len();
        if !removes.is_empty(){
            let mut merge_removes = vec![removes.first().expect("").clone()];
            let mut last = merge_removes.last_mut().expect("");
            for i in 1..removes.len() {
                if last.1 == removes[i].0{
                    last.1 = removes[i].1;
                }else{
                    merge_removes.push(removes[i]);
                    last = merge_removes.last_mut().expect("");
                }
            }

            std::mem::drop(removes);
            for it in merge_removes.iter().rev() {
                self.remove_start_end(it.0 as usize, it.1 as usize);
            }
            self.set_len(self.len() - will_remove as u32);
        }

        will_remove as LenType
    }

    pub fn rem_one(&mut self, offet: usize, value_len: LenType) {
        let mut p = self.0[offet..].as_mut_ptr();
        let t = offet + value_len as usize + BYTES_LEN_TYPE;
        unsafe { ptr::copy(p.offset(t as isize), p, self.0.len() - t); }
        self.0.truncate(self.0.len() - value_len as usize - BYTES_LEN_TYPE);
    }

    pub fn remove_start_end(&mut self, start: usize, end: usize) {
        let mut p = self.0[start..].as_mut_ptr();
        unsafe { ptr::copy(p.offset(end as isize), p, self.0.len() - end); }
        self.0.truncate(self.0.len() - (end - start));
    }

    pub fn index<'a>(&'a self, index: i32) -> Option<&'a [u8]> {
        let offset = self.get_offset_index(index as usize);
        if let Some(offset) = offset {
            let node = ZipListNode::read_value(&self.0,offset);
            Some(node)
        } else {
            None
        }
    }

    pub fn range(&self, start: i32, stop: i32) -> Vec<Vec<u8>> {
        let len = stop - start + 1;
        let mut result = Vec::with_capacity(len as usize);
        let mut index = 0;
        let mut it = ZipListIter::new(self);
        loop{
            let node = it.next();
            if let Some(node) = node {
                if index >= start && index <= stop {
                    result.push(node.value().to_vec());
                }
            }else{
                break
            }
            index += 1;
        }

        result
    }

    pub fn count_index(len: i32, index: i32) -> i32 {
        let result_index = {
            if index < 0 {
                let mut index_ = len + index;
                if index_ < 0 {
                    index_ = 0;
                }
                index_
            } else {
                if index >= len {
                    len - 1
                } else {
                    index
                }
            }
        };
        result_index
    }

    /// 返回值 (start_in_index, stop_in_index)
    pub fn count_in_index(len: LenType, offset: usize, start_index: usize, stop_index: usize) -> Option<(usize, usize)> {
        let len = len as usize;
        let mut start_in_index = 0usize;
        let mut stop_in_index = 0usize;

        if start_index >= len + offset || stop_index <= offset {
            return None;
        }

        start_in_index = start_index - offset;
        stop_in_index = start_in_index + (stop_index - start_index) + 1;
        if stop_in_index >= len {
            stop_in_index = len - 1;
        }
        Some((start_in_index, stop_in_index))
    }
}

struct ZipListIter<'a> {
    zip_list: &'a [u8],
    len: LenType,
    start_cur: usize,
}

impl<'a> ZipListIter<'a> {
    pub fn new(zip: &'a ZipList) -> Self {
        ZipListIter {
            zip_list: &zip.0[ZipList::OFFSET_VALUE..],
            len: zip.len(),
            start_cur: 0,
        }
    }

    pub fn offset(&self) -> usize {
        self.start_cur
    }

    fn prev_offset(&self) -> Option<usize> {
        if self.start_cur >= ZipListNode::LEN_NODE * 2 {
            let len_value = ZipListNode::read_bytes_of_value(&self.zip_list[self.start_cur - ZipListNode::LEN_NODE * 2..]);
            let mut cur = self.start_cur;
            cur -= len_value + ZipListNode::LEN_NODE * 2;
            Some(cur)
        } else {
            None
        }
    }

    fn next_offset(&self) -> Option<usize> {
        if self.start_cur < self.zip_list.len() {
            let len_value = ZipListNode::read_bytes_of_value(self.zip_list);
            let mut cur = self.start_cur;
            cur += len_value + ZipListNode::LEN_NODE * 2;
            Some(cur)
        } else {
            None
        }
    }

    fn prev(&mut self) -> Option<ZipListNode<'a>> {
        if self.start_cur >= ZipListNode::LEN_NODE * 2 {
            let len_value = ZipListNode::read_bytes_of_value(&self.zip_list[self.start_cur - ZipListNode::LEN_NODE * 2..]);
            let cur = self.start_cur;
            self.start_cur -= len_value + ZipListNode::LEN_NODE * 2;
            ZipListNode::from_start(&self.zip_list[self.start_cur..cur])
        } else {
            None
        }
    }

    fn next_back(&mut self) -> Option<ZipListNode<'a>> {
        let mut p = self.start_cur - ZipListNode::LEN_NODE;
        if p < 0 {
            None
        } else {
            let len_value = ZipListNode::read_bytes_of_value(&self.zip_list[p..]);
            p -= len_value + ZipListNode::LEN_NODE;
            if p < 0 {
                None
            } else {
                (p, self.start_cur) = (self.start_cur, p);
                ZipListNode::from_start(&self.zip_list[self.start_cur..p])
            }
        }
    }
}

impl<'a> Iterator for ZipListIter<'a> {
    type Item = ZipListNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_cur < self.zip_list.len() {
            let len_value = ZipListNode::read_bytes_of_value(self.zip_list);
            let cur = self.start_cur;
            self.start_cur += len_value + ZipListNode::LEN_NODE * 2;
            ZipListNode::from_start(&self.zip_list[cur..self.start_cur])
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {
    use crate::redis_rocksdb::zip_list::{ZipList, ZipListIter};

    #[test]
    fn test_sub_uzise() {
        let a = (||{
            return 4usize;
        })();
        let b = 10usize;
        let t = a.wrapping_sub(b);
        println!("{}\n{}", t, usize::MAX);
    }

    #[test]
    fn test_zip_list_iterator() {
        let mut zip = ZipList::new();
        zip.push_left(&[1, 2, 3]);
        zip.push_left(&[4, 5, 6]);
        zip.push_left(&[7, 8, 9]);

        let it = ZipListIter::new(&zip);
    }
}