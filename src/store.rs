use crate::model::KvdResult;
use std::path::PathBuf;

struct Store {}

impl Store {
    pub fn open(path: PathBuf) -> KvdResult<Self> {
        Ok(Store {})
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KvdResult<()> {
        Ok(())
    }

    pub fn get(&self, key: Vec<u8>) -> KvdResult<Option<Vec<u8>>> {
        Ok(None)
    }

    pub fn del(&mut self, key: Vec<u8>) -> KvdResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_open() {
        let path = PathBuf::from("./data/data.wal");
        let store = Store::open(path).unwrap();
    }

    fn get_test_store() -> Store {
        let path: PathBuf = PathBuf::from("tmp");
        Store::open(path).unwrap()
    }
}
