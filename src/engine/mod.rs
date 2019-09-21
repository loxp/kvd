pub mod bitcask;
pub mod mock;

use crate::model::KvdResult;

pub trait Engine {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KvdResult<()>;
    fn get(&mut self, key: Vec<u8>) -> KvdResult<Option<Vec<u8>>>;
    fn del(&mut self, key: Vec<u8>) -> KvdResult<()>;
}
