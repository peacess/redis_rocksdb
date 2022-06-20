use crate::{Bytes, LenType, RrError};

pub trait RedisHash {
    fn hget<K: Bytes, V: Bytes>(&self, key: &K) -> Result<Option<Vec<u8>>, RrError>;
    fn hset<K: Bytes, V: Bytes>(&mut self, key: &K, value: &V) -> Result<bool, RrError>;
    fn hlen<K: Bytes>(&mut self, key: &K) -> Result<LenType, RrError>;
}
