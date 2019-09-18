use crate::model;
use crate::model::KvdErrorKind::KeyNotFound;
use crate::model::{KvdError, KvdErrorKind, KvdResult};
use crate::store::Store;
use std::fs::File;
use std::io;
use std::io::BufRead;
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

            let cmd = request
                .get(0)
                .ok_or(KvdError::from(KvdErrorKind::KeyNotFound))?;
            let set = Vec::from("set");
            let get = Vec::from("get");
            let del = Vec::from("del");
            if cmd == &get {
                let key = request
                    .get(1)
                    .ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                let value = self.store.get(key.clone())?.unwrap_or(Vec::new());
                println!("{:?}", str::from_utf8(&value)?);
            } else if cmd == &set {
                let key = request
                    .get(1)
                    .ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                let value = request
                    .get(2)
                    .ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                let result = self.store.set(key.clone(), value.clone())?;
                println!("OK");
            } else if cmd == &del {
                let key = request
                    .get(1)
                    .ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                let result = self.store.del(key.clone())?;
                println!("OK");
            } else {
                println!("invalid command");
            }

            // TODO: why match Vec cannot work?

            /*  match cmd.clone() {
                get => {
                    let key = request.get(1).ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                    println!("{:?}", self.store.get(key.clone())?);
                }
                set => {
                    let key = request.get(1).ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                    let value = request.get(2).ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                    let result = self.store.set(key.clone(), value.clone())?;
                    println!("OK");
                }
                del => {
                    let key = request.get(1).ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;
                    let result = self.store.del(key.clone())?;
                    println!("OK");
                }
                _ => {
                    println!("invalid command");
                }
            }*/
        }
        Ok(())
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
