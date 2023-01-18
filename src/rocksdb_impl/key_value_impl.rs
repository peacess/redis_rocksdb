use crate::{Bytes, KeyValue, RedisRocksdb, RrError};

impl KeyValue for RedisRocksdb {
    fn get<K: Bytes, V: Bytes>(&self, key: &K) -> Result<Option<Vec<u8>>, RrError> {
        let v = self.db.get(key.as_ref())?;
        match v {
            None => Ok(None),
            Some(v) => Ok(Some(v.to_vec()))
        }
    }

    fn put<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<(), RrError> {
        match self.db.put(key.as_ref(), value.as_ref()) {
            Ok(t) => Ok(t),
            Err(e) => Err(RrError::from(e))
        }
    }
}