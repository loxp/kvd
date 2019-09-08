use crate::model::{KvdError, KvdResult};
use std::collections::HashMap;
use std::path::PathBuf;

struct Store {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl Store {
    const DEFAULT_INITIAL_CAPACITY: usize = 128;
}

impl Store {
    pub fn open(path: PathBuf) -> KvdResult<Self> {
        Ok(Store {
            data: HashMap::with_capacity(Store::DEFAULT_INITIAL_CAPACITY),
        })
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KvdResult<()> {
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: Vec<u8>) -> KvdResult<Option<&Vec<u8>>> {
        let value = self.data.get(&key);
        Ok(value)
    }

    pub fn del(&mut self, key: Vec<u8>) -> KvdResult<()> {
        let result = self.data.remove(&key);
        match result {
            Some(_) => Ok(()),
            None => Err(KvdError::KeyNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_open() {
        get_test_store();
    }

    #[test]
    fn test_set() {
        let mut store = get_test_store();
        let result = store.set(Vec::from("key"), Vec::from("value"));
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_get() {
        let mut store = get_test_store();
        let result = store.get(Vec::from("key"));
        assert_eq!(Ok(None), result);
    }

    #[test]
    fn test_del() {
        let mut store = get_test_store();
        let result = store.del(Vec::from("key"));
        assert_eq!(Err(KvdError::KeyNotFound), result);
    }

    #[test]
    fn test_set_then_get_then_del_then_get() {
        let mut store = get_test_store();

        // define test data
        let key = Vec::from("key");
        let value = Vec::from("value");

        // get, set and get, del and get
        let result = store.get(key.clone());
        assert_eq!(Ok(None), result);
        let result = store.set(key.clone(), value.clone());
        assert_eq!(Ok(()), result);
        let result = store.get(key.clone());
        assert_eq!(Ok(Some(&value)), result);
        let result = store.del(key.clone());
        assert_eq!(Ok(()), result);
        let result = store.get(key.clone());
        assert_eq!(Ok(None), result);
    }

    fn get_test_store() -> Store {
        let path = get_tmp_store_path();
        Store::open(path).unwrap()
    }

    fn get_tmp_store_path() -> PathBuf {
        let time = SystemTime::now();
        let time = time.duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path_str = format!("/tmp/kvd_store_{}.wal", time);
        PathBuf::from(path_str)
    }
}
