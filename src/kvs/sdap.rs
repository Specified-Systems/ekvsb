use crate::kvs::KeyValueStore;
use crate::task::Existence;
use crate::Result;

#[derive(Debug)]
pub struct SdapCache {
    cache: cache::Cache,
}

impl SdapCache {
    pub fn new() -> Self {
        Self {
            cache: cache::Cache::new()
        }
    }
}

impl KeyValueStore for SdapCache {
    type OwnedValue = Vec<u8>;

    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<Existence> {
        track_any_err!(self.cache.put(key, value));
        Ok(Existence::unknown())
    }

    fn get(&mut self, key: &[u8]) -> Result<Option<Self::OwnedValue>> {
        let value = track_any_err!(self.cache.get(key))?;
        Ok(value)
    }

    fn delete(&mut self, key: &[u8]) -> Result<Existence> {
        track_any_err!(self.cache.delete(key))?;
        Ok(Existence::unknown())
    }
}
