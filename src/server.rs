use crate::model::KvdResult;
use crate::store::Store;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct Server {
    store: Store,
}

impl Server {
    pub fn new(path: &str) -> KvdResult<Server> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(path)).unwrap();

        let wal_dir = settings.get_str("wal_dir")?;

        let store = Store::open(PathBuf::from(wal_dir))?;
        let server = Server { store };

        Ok(server)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_server() {
        let server = Server::new("conf/kvd.yml");
    }
}
