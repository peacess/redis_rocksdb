
use crate::{LenType, Object, RrError, WrapDb};
use crate::rocksdb_impl::shared::{get_field_from_key, make_field_key};

/// 直接使用key + field的方式，把value的值存入数据库中
/// 当获取所有field或值时需要使用 prefix_iterator，这时性能不如 [crate::BitObject]
pub struct ObjectImp {}

impl<T: WrapDb> Object<T> for ObjectImp {
    fn del(&self, t: &T, key: &[u8], field: &[u8]) -> Result<(), RrError> {
        let new_key = make_field_key(key, field);
        t.delete(&new_key)?;
        Ok(())
    }

    fn dels(&self, t: &T, key: &[u8], fields: &[&[u8]]) -> Result<LenType, RrError> {
        let mut count = 0;
        for f in fields {
            let new_key = make_field_key(key, f);
            t.delete(&new_key)?;
            count += 1;
        }
        Ok(count)
    }

    fn exists(&self, t: &T, key: &[u8], field: &[u8]) -> Result<bool, RrError> {
        let new_key = make_field_key(key, field);
        let old = t.get(&new_key)?;
        Ok(old.is_some())
    }

    fn get(&self, t: &T, key: &[u8], field: &[u8]) -> Result<Option<Vec<u8>>, RrError> {
        let new_key = make_field_key(key, field);
        let v = t.get(&new_key)?;
        return Ok(v);
    }

    fn get_all(&self, t: &T, key: &[u8]) -> Result<Option<Vec<(Vec<u8>, Vec<u8>)>>, RrError> {
        let mut re = Vec::with_capacity(10);
        let new_key = make_field_key(key, &[]);
        let it = t.prefix_iterator(&new_key);
        for k in it {
            let kk = k?;
            let field_key = get_field_from_key(key, &kk.0);
            re.push((field_key.to_vec(), kk.1.to_vec()));
        }
        if re.is_empty() {
            Ok(None)
        } else {
            Ok(Some(re))
        }
    }

    fn keys(&self, t: &T, key: &[u8]) -> Result<Option<Vec<Vec<u8>>>, RrError> {
        let mut re = Vec::with_capacity(10);
        let new_key = make_field_key(key, &[]);
        let it = t.prefix_iterator(&new_key);
        for k in it {
            let kk = k?;
            let field_key = get_field_from_key(key, &kk.0);
            re.push(field_key.to_vec());
        }
        if re.is_empty() {
            Ok(None)
        } else {
            Ok(Some(re))
        }
    }

    fn len(&self, t: &T, key: &[u8]) -> Result<Option<LenType>, RrError> {
        let new_key = make_field_key(key, &[]);
        let it = t.prefix_iterator(&new_key);
        let l = it.count();
        if l == 0 {
            return Ok(None);
        } else {
            Ok(Some(l as LenType))
        }
    }

    fn mget(&self, t: &T, key: &[u8], fields: &[&[u8]]) -> Result<Vec<Option<Vec<u8>>>, RrError> {
        let mut values = Vec::with_capacity(fields.len());
        for f in fields {
            let new_key = make_field_key(key, f);
            if let Some(v) = t.get(&new_key)? {
                values.push(Some(v));
            } else {
                values.push(None);
            }
        }
        Ok(values)
    }

    fn set(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<(), RrError> {
        let new_key = make_field_key(key, field);
        t.put(&new_key, value)?;
        Ok(())
    }

    fn set_not_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_field_key(key, field);
        if let None = t.get(&new_key)? {
            t.put(&new_key, value)?;
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn set_exist(&self, t: &T, key: &[u8], field: &[u8], value: &[u8]) -> Result<i32, RrError> {
        let new_key = make_field_key(key, field);
        if let Some(_) = t.get(&new_key)? {
            t.put(&new_key, value)?;
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    fn vals(&self, t: &T, key: &[u8]) -> Result<Vec<Vec<u8>>, RrError> {
        let mut re = Vec::with_capacity(10);
        let new_key = make_field_key(key, &[]);
        let it = t.prefix_iterator(&new_key);
        for k in it {
            let kk = k?;
            re.push(kk.1.to_vec());
        }
        Ok(re)
    }

    fn del_key(&self, t: &T, key: &[u8]) -> Result<(), RrError> {
        let new_key = make_field_key(key, &[]);
        let it = t.prefix_iterator(&new_key);
        for k in it {
            let kk = k?;
            t.delete(&kk.0)?;
        }
        Ok(())
    }
}

