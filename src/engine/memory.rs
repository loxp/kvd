use super::KvdEngine;
use crate::model::KvdResult;
use std::collections::HashMap;

pub struct MemoryEngine {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemoryEngine {
    pub fn new() -> MemoryEngine {
        MemoryEngine {
            map: HashMap::new(),
        }
    }
}

impl KvdEngine for MemoryEngine {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KvdResult<()> {
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&mut self, key: Vec<u8>) -> KvdResult<Option<Vec<u8>>> {
        Ok(self.map.get(&key).cloned())
    }

    fn del(&mut self, key: Vec<u8>) -> KvdResult<()> {
        self.map.remove(&key);
        Ok(())
    }
}
