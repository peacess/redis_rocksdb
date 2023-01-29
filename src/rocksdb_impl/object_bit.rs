use std::{mem, ptr, slice};

use crate::{LenType, Object, read_int, read_int_ptr, RrError, WrapDb, write_int_ptr};
use crate::rocksdb_impl::shared::{make_head_key, make_key};

/// 这个对应redis中的hash, 字段数据量建议在2048个以内，在遍历数据时，性能比[ObjectImp]好
/// 使用一个大的数组把key的值存下，以方便访问全部的field，与[ObjectImp]相比需要维护一个数组，当field数量不多时，性能比较好
/// 所有的key以数组方式存入，有先后关系。如果删除其中的一个field其后的数据，会平移。
///
/// 数据特点：
/// 有field变动的性能，是O(N)，所以数据不能多
/// 读取数据时O(1)
pub struct BitObject {}

impl<T: WrapDb> Object<T> for BitObject {
    fn del(&self, t: &T, key: &[u8], field: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let mut f = BitField::new(fv);
            f.del(field);
            t.put(&head_key, &f.data)?;
        }
        let new_key = make_key(key, field);
        t.delete(&new_key)?;

        Ok(())
    }

    fn dels(&self, t: &T, key: &[u8], fields: &[&[u8]]) -> Result<LenType, RrError> {
        let mut count = 0;
        for f in fields {
            let new_key = make_key(key, f);
            t.delete(&new_key)?;
        }
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let mut f = BitField::new(fv);
            for field in fields {
                if f.del(field) {
                    count += 1;
                }
            }
            t.put(&head_key, &f.data)?;
        }

        Ok(count)
    }

    fn exists(&self, t: &T, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        let new_key = make_key(key, field);
        let old = t.get(&new_key)?;
        Ok(old.is_some())
    }

    fn get(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        let new_key = make_key(key, field);
        let v = t.get(&new_key)?;
        return Ok(v);
    }

    fn get_all(&self, t: &T, key: &[u8]) -> Result<Option<Vec<(Vec<u8>, Vec<u8>)>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitField::new(fv);
            let mut re = Vec::with_capacity(few_field.len());
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                let v = t.get(&new_key)?;
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

    fn keys(&self, t: &T, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitField::new(fv);
            let mut re = Vec::with_capacity(few_field.len());
            for field in few_field.new_field_it() {
                re.push(field.field.to_vec());
            }
            Ok(Some(re))
        } else {
            return Ok(None);
        }
    }

    fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitField { data: fv };
            Ok(Some(few_field.len() as LenType))
        } else {
            return Ok(None);
        }
    }

    fn mget(&self, t: &T, key: &[u8], fields: &[&[u8]]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        let mut values = Vec::with_capacity(fields.len());
        for f in fields {
            let new_key = make_key(key, f);
            if let Some(v) = t.get(&new_key)? {
                values.push(Some(v));
            } else {
                values.push(None);
            }
        }
        Ok(values)
    }

    fn set(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let mut few_field = BitField::new(fv);
            few_field.set(field);
            t.put(&head_key, &few_field.data)?;
        } else {
            let mut few_field = BitField::new(vec![]);
            few_field.set(field);
            t.put(&head_key, &few_field.data)?;
        }
        let new_key = make_key(key, field);
        t.put(&new_key, value)?;
        Ok(())
    }

    fn set_not_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_key(key, field);
        if let None = t.get(&new_key)? {
            t.put(&new_key, value)?;

            let head_key = make_head_key(key);
            if let Some(fv) = t.get(&head_key)? {
                let mut few_field = BitField::new(fv);
                few_field.set(field);
                t.put(&head_key, &few_field.data)?;
            } else {
                let mut few_field = BitField::new(vec![]);
                few_field.set(field);
                t.put(&head_key, &few_field.data)?;
            }

            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn set_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_key(key, field);
        if let Some(_) = t.get(&new_key)? {
            t.put(&new_key, value)?;
            //由于key是存在的，所以这里不用再修 head key了
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn vals(&self, t: &T, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitField::new(fv);
            let mut re = Vec::with_capacity(few_field.len());
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                let v = t.get(&new_key)?;
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

    fn remove_key(&self, t: &T, key: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitField::new(fv);
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                t.delete(&new_key)?;
            }
            t.delete(&head_key)?;
        }
        return Ok(());
    }
}

///所有的field连续存入一遍连续的内存区中
pub(crate) struct BitField {
    pub(crate) data: Vec<u8>,
}

//每一个字段的byte数的类型
pub(crate) type SizeBitField = i32;
//字段个数（len）的类型
pub(crate) type LenBitField = u64;

impl BitField {
    const SIZE: usize = mem::size_of::<SizeBitField>();

    pub fn new(data: Vec<u8>) -> Self {
        let mut data = data;
        if data.is_empty() {
            data.resize(mem::size_of::<LenBitField>(), 0);
        }
        BitField { data }
    }

    /// 返回值true: 字段存在
    pub fn del(&mut self, field: &[u8]) -> bool {
        let (start, field_size) = self.find(field);
        if start >= 0 {
            let end = start + BitField::SIZE as isize + field_size as isize;
            let p = self.data.as_ptr();
            unsafe {
                ptr::copy(p.offset(end), p.offset(start).cast_mut(), self.data.len() - end as usize);
                self.data.set_len(self.len() - field_size as usize - BitField::SIZE);
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
            let add = BitField::SIZE + field.len();
            self.data.reserve(add);
            unsafe {
                let p = self.data.as_mut_ptr().offset(self.len() as isize - add as isize);
                //写入字段的bytes数量
                write_int_ptr(p, field.len() as SizeBitField);
                //写入字段
                ptr::copy_nonoverlapping(field.as_ptr(), p.offset(BitField::SIZE as isize), field.len());
                let len = self.len() + 1;
                //写入总的字段个数
                write_int_ptr(self.data.as_mut_ptr(), len as LenBitField);
            }
            false
        }
    }

    pub fn len(&self) -> usize {
        let l = read_int::<LenBitField>(&self.data);
        return l as usize;
    }

    /// 返回值 0： 开始偏移，如果没有找到为-1
    /// 返回值 1： field size
    fn find(&self, field: &[u8]) -> (isize, usize) {
        let l = self.len();
        let p = self.data.as_ptr();
        let mut offset = mem::size_of::<LenBitField>() as isize;
        unsafe {
            for _i in 0..l {
                let field_size = read_int_ptr::<SizeBitField>(p.offset(offset)) as usize;
                offset += BitField::SIZE as isize;
                let f = slice::from_raw_parts(p.offset(offset), field_size);
                if f == field {
                    let start = offset - BitField::SIZE as isize;
                    return (start, field_size);
                }
                //指向下一个field
                offset += field_size as isize;
            }
        }
        return (-1, 0);
    }

    pub(crate) fn new_field_it(&self) -> BitFieldIt {
        BitFieldIt::new(self)
    }
}

pub(crate) struct BitFieldIt<'a> {
    data: &'a BitField,
    len: isize,
    index: isize,
    offset: isize,
}

impl<'a> BitFieldIt<'a> {
    pub fn new(d: &'a BitField) -> Self {
        BitFieldIt {
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

impl<'a> Iterator for BitFieldIt<'a> {
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
            self.offset = mem::size_of::<LenBitField>() as isize;
        }

        self.index += 1;
        let field_size = read_int_ptr::<SizeBitField>(unsafe { self.data.data.as_ptr().offset(self.offset) });
        let it = FieldItValue {
            field: unsafe { slice::from_raw_parts(self.data.data.as_ptr().offset(self.offset + BitField::SIZE as isize), field_size as usize) },
        };
        return Some(it);
    }
}




