use std::ptr;

use rocksdb::{Transaction, TransactionDB};

use crate::{KvSet, KvSetTr, RrError};

pub struct KvSetImp {}

impl KvSet for KvSetImp {
    fn kv_set_del(&self, db: &TransactionDB, key: &[u8], field: &[u8]) -> Result<(), RrError> {
        todo!()
    }

    fn kv_set_dels(&self, db: &TransactionDB, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError> {
        todo!()
    }

    fn kv_set_exists(&self, db: &TransactionDB, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        todo!()
    }

    fn kv_set_get(&self, db: &TransactionDB, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        todo!()
    }

    fn kv_set_get_all(&self, db: &TransactionDB, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_keys(&self, db: &TransactionDB, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_len(&self, db: &TransactionDB, key: &[u8]) -> Result<Option<i64>, RrError> {
        todo!()
    }

    fn kv_set_mget(&self, db: &TransactionDB, key: &[u8], fields: &[u8]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_set(&self, db: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        todo!()
    }

    fn kv_set_set_not_exist(&self, db: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn kv_set_set_exist(&self, db: &TransactionDB, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn kv_set_vals(&self, db: &TransactionDB, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        todo!()
    }

    fn kv_set_remove_key(&self, db: &TransactionDB, key: &[u8]) -> Result<(), RrError> {
        todo!()
    }
}

/// 这个集合适合字段数量比较少时使用，
/// 实现，把所有的字段名存放到一个key中，这样方便于对整个字段的管理，同样也会产生一个问题，就是不要有太多的字段
/// 每个字段的key生成方式为，为key生成一个唯一的id, 这样解决kv数据库中k冲突的问题
struct KvSetTrImp {}

impl KvSetTr for KvSetTrImp {
    fn kv_set_del(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8]) -> Result<(), RrError> {
        let new_key = make_key(key,field);
        tr.delete(&new_key)?;
        Ok(())
    }

    fn kv_set_dels(&self, tr: &Transaction<TransactionDB>, key: &[u8], fields: &[&[u8]]) -> Result<i64, RrError> {
        todo!()
    }

    fn kv_set_exists(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        todo!()
    }

    fn kv_set_get(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        let new_key = make_key(key, field);
        let v = tr.get(&new_key)?;
        return Ok(v);
    }

    fn kv_set_get_all(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_keys(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_len(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Option<i64>, RrError> {
        todo!()
    }

    fn kv_set_mget(&self, tr: &Transaction<TransactionDB>, key: &[u8], fields: &[u8]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        todo!()
    }

    fn kv_set_set(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        let new_key = make_key(key,field);
        tr.put(&new_key, value)?;
        Ok(())
    }

    fn kv_set_set_not_exist(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn kv_set_set_exist(&self, tr: &Transaction<TransactionDB>, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        todo!()
    }

    fn kv_set_vals(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        todo!()
    }

    fn kv_set_remove_key(&self, tr: &Transaction<TransactionDB>, key: &[u8]) -> Result<(), RrError> {
        todo!()
    }
}

fn make_key(key: &[u8], field: &[u8]) -> Vec<u8> {
    let mut new_key = Vec::with_capacity(key.len() + field.len() + 3);
    unsafe {//这里使用性能更高的 copy_nonoverlapping
        let mut p = new_key.as_mut_ptr();
        ptr::copy_nonoverlapping(key.as_ptr(), p, key.len());
        p = p.offset(key.len() as isize);
        *p = ':' as u8;
        *(p.offset(1)) = '_' as u8;
        *(p.offset(2)) = '_' as u8;
        p = p.offset(3);
        ptr::copy_nonoverlapping(field.as_ptr(), p, field.len());
    }
    return new_key;
}
