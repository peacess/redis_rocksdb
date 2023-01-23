use rocksdb::{Transaction, TransactionDB};

use crate::{Object, RrError};
use crate::rocksdb_impl::{BitField, make_head_key, make_key};

/// 这个集合适合字段数量比较少时使用，
/// 实现，把所有的字段名存放到一个key中，这样方便于对整个字段的管理，同样也会产生一个问题，就是不要有太多的字段
/// 每个字段的key生成方式为，为key生成一个唯一的id, 这样解决kv数据库中k冲突的问题
pub struct FewObjectTrans {}

impl<'db> Object<Transaction<'db, TransactionDB>> for FewObjectTrans {
    fn del(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], field: &[u8]) -> Result<(), RrError> {
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

    fn dels(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError> {
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

    fn exists(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        let new_key = make_key(key, field);
        let old = t.get(&new_key)?;
        Ok(old.is_some())
    }

    fn get(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        let new_key = make_key(key, field);
        let v = t.get(&new_key)?;
        return Ok(v);
    }

    fn get_all(&self, t: &Transaction<'db, TransactionDB>, key: &[u8]) -> Result<Option<Vec<(Vec<u8>, Vec<u8>)>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitField::new(fv);
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

    fn keys(&self, t: &Transaction<'db, TransactionDB>, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
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

    fn len(&self, t: &Transaction<'db, TransactionDB>, key: &[u8]) -> Result<Option<i64>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitField { data: fv };
            Ok(Some(few_field.len() as i64))
        } else {
            return Ok(None);
        }
    }

    fn mget(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], fields: &[&[u8]]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
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

    fn set(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
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

    fn set_not_exist(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_key(key, field);
        if let None = t.get(&new_key)? {
            t.put(new_key, value)?;

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

    fn set_exist(&self, t: &Transaction<'db, TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_key(key, field);
        if let Some(_) = t.get(&new_key)? {
            t.put(new_key, value)?;
            //由于key是存在的，所以这里不用再修 head key了
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn vals(&self, t: &Transaction<'db, TransactionDB>, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(head_key)? {
            let few_field = BitField::new(fv);
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

    fn remove_key(&self, t: &Transaction<'db, TransactionDB>, key: &[u8]) -> Result<(), RrError> {
        let head_key = make_head_key(key);
        if let Some(fv) = t.get(&head_key)? {
            let few_field = BitField::new(fv);
            for field in few_field.new_field_it() {
                let new_key = make_key(key, field.field);
                t.delete(new_key)?;
            }
            t.delete(&head_key)?;
        }
        return Ok(());
    }
}
