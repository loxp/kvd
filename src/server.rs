use crate::model;
use crate::model::KvdErrorKind::KeyNotFound;
use crate::model::{KvdError, KvdErrorKind, KvdResult};
use crate::store::Store;
use std::fs::File;
use std::io;
use std::io::{stdin, BufRead};
use std::path::{Path, PathBuf};
use std::str;

pub struct Server {
    store: Store,
}

impl Server {
    pub fn new(path: &str) -> KvdResult<Server> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(path))?;

        let wal_dir = settings.get_str("wal_dir")?;
        let store = Store::open(PathBuf::from(wal_dir))?;
        let server = Server { store };

        Ok(server)
    }

    pub fn serve(&mut self) -> KvdResult<()> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            let request = model::parse_request_from_line(line)?;
            let result = self.dispatch_request(request);
            match result {
                Ok(r) => println!(
                    "{:?}",
                    str::from_utf8(r.as_slice()).unwrap_or("not a utf-8 value")
                ),
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(())
    }

    /// return a str in Vec for display
    fn dispatch_request(&mut self, request: Vec<Vec<u8>>) -> KvdResult<Vec<u8>> {
        let cmd = request
            .get(0)
            .ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;

        let result = match cmd.as_slice() {
            b"get" => self.handle_get(request).map(|r| r.unwrap_or(Vec::new())),
            b"set" => self.handle_set(request).and(Ok(Vec::new())),
            b"del" => self.handle_del(request).and(Ok(Vec::new())),
            _ => Err(KvdError::from(KvdErrorKind::InvalidRequest)),
        };

        result
    }

    // TODO: is it right to return a nil Vec when key is not found?
    fn handle_get(&mut self, request: Vec<Vec<u8>>) -> KvdResult<Option<Vec<u8>>> {
        if request.len() != 2 {
            return Err(KvdError::from(KvdErrorKind::InvalidRequest));
        }
        let result = self.store.get(request.get(1).unwrap().clone())?;
        Ok(result)
    }

    fn handle_set(&mut self, request: Vec<Vec<u8>>) -> KvdResult<()> {
        if request.len() != 3 {
            return Err(KvdError::from(KvdErrorKind::InvalidRequest));
        }
        self.store.set(
            request.get(1).unwrap().clone(),
            request.get(2).unwrap().clone(),
        )
    }

    fn handle_del(&mut self, request: Vec<Vec<u8>>) -> KvdResult<()> {
        if request.len() != 2 {
            return Err(KvdError::from(KvdErrorKind::InvalidRequest));
        }
        self.store.del(request.get(1).unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_server() {
        let server = Server::new("conf/default.yml").unwrap();
    }
}
