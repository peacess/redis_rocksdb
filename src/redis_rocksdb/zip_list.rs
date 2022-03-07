use core::{mem, ptr};

use ckb_rocksdb::{ReadOptions, Transaction, TransactionDB};
use ckb_rocksdb::prelude::Get;

use crate::{EndianScalar, Error, LenType, read_int, read_len_type, SIZE_LEN_TYPE, write_int};

///
/// ```rust
/// use redis_rocksdb::{LenType, MetaKey};
///
/// struct ZipList{
///     len: LenType,
///     values: [u8],
/// }
/// ```
pub(crate) struct ZipList(Vec<u8>);

impl From<Vec<u8>> for ZipList {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes;
        if bytes.len() < ZipList::len_init {
            bytes.resize(ZipList::len_init, 0);
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
    const len_init: usize = 32;
    const offset_value: usize = SIZE_LEN_TYPE;
    pub fn new() -> Self {
        ZipList(Vec::from([0; ZipList::len_init]))
    }

    #[inline]
    fn write_value(value: &[u8], p: *mut u8) {
        let x_le = (value.len() as LenType).to_little_endian();
        unsafe {
            ptr::copy_nonoverlapping(
                &x_le as *const LenType as *const u8,
                p,
                SIZE_LEN_TYPE,
            );
            ptr::copy_nonoverlapping(
                value.as_ptr(),
                p.offset(SIZE_LEN_TYPE as isize),
                value.len(),
            );
        }
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

    pub fn push_left(&mut self, value: &[u8]) {
        self.insert_left(0, value);
    }

    pub fn push_right(&mut self, value: &[u8]) {
        let s = self.len() + 1;
        self.set_len(s);

        let old_len = self.0.len();
        let add_len = SIZE_LEN_TYPE + value.len();

        // fn resize will set the default value, so replace with  reserve and set_len
        self.0.reserve(add_len);
        unsafe { self.0.set_len(old_len + add_len); }
        unsafe { ZipList::write_value(value, self.0.as_mut_ptr().offset(old_len as isize)); }
    }

    pub fn insert_left(&mut self, index: i32, value: &[u8]) {
        // todo if the index > value_len
        let s = self.len() + 1;
        self.set_len(s);

        let old_len = self.0.len();
        let add_len = SIZE_LEN_TYPE + value.len();
        self.0.reserve(add_len);
        unsafe { self.0.set_len(old_len + add_len); }
        unsafe {
            let offset = {
                let mut offset = 0;
                if index < self.len() as i32 - 1 {
                    for _ in 1..index + 1 {
                        let value_len = read_len_type(&self.0[offset..]) as usize;
                        offset += value_len + SIZE_LEN_TYPE;
                    }
                    let p = self.0.as_mut_ptr().offset(offset as isize);
                    // Shift everything over to make space. (Duplicating the
                    // `index`th element into two consecutive places.)
                    ptr::copy(p, p.offset(add_len as isize), old_len - add_len);
                }// else时，相当于 push_right
                offset
            };

            ZipList::write_value(value, self.0.as_mut_ptr().offset(offset as isize));
        }
    }

    pub fn insert_right(&mut self, index: i32, value: &[u8]) {
        self.insert_left(index + 1, value)
    }

    pub fn set(&mut self, index: i32, value: &[u8]) {
        // todo if the index > value_len
        unsafe {
            let offset = {
                let mut offset = 0;
                let mut old_value_len: usize = 0;
                for _ in 0..index + 1 {
                    old_value_len = read_len_type(&self.0[offset..]) as usize;
                    offset += old_value_len + SIZE_LEN_TYPE;
                }
                offset -= old_value_len + SIZE_LEN_TYPE;

                let p = self.0.as_mut_ptr().offset(offset as isize);
                //这里一定要使用isize,因为可能为负数
                let diff: isize = value.len() as isize - (old_value_len as isize);
                if diff == 0 {
                    //这种情况下 不需要移动任何数据，因为大小是一样的
                } else if diff > 0 {
                    self.0.reserve(diff as usize);
                    self.0.set_len(self.0.len() + diff as usize);
                    ptr::copy(p, p.offset(diff as isize), self.0.len() - offset - diff as usize);
                } else if diff < 0 {
                    ptr::copy(p, p.offset(diff), self.0.len() - offset);
                    self.0.truncate(self.0.len() - diff as usize);
                }
                offset as isize
            };

            ZipList::write_value(value, self.0.as_mut_ptr().offset(offset));
        }
    }

    pub fn index(&self, index: i32) -> Option<&[u8]> {
        if index >= self.len() as i32 {
            return None;
        }
        let mut offset = ZipList::offset_value;
        for i in 0..index {
            let size_value = read_len_type(&self.0[offset..]);
            offset += SIZE_LEN_TYPE + size_value as usize;
        }
        let size_value = read_len_type(&self.0[offset..]);
        Some(&self.0[offset + SIZE_LEN_TYPE..offset + SIZE_LEN_TYPE + size_value as usize])
    }
}
