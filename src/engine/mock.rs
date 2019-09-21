use super::KvdEngine;
use crate::model::KvdResult;

pub struct MockEngine {}

impl MockEngine {
    pub fn new() -> MockEngine {
        MockEngine {}
    }
}

impl KvdEngine for MockEngine {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KvdResult<()> {
        Ok(())
    }

    fn get(&mut self, key: Vec<u8>) -> KvdResult<Option<Vec<u8>>> {
        Ok(None)
    }

    fn del(&mut self, key: Vec<u8>) -> KvdResult<()> {
        Ok(())
    }
}
