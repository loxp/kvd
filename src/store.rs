use crate::model::{KvdError, KvdResult};
use std::path::{PathBuf, Path};
use std::fs::{File, OpenOptions};
use std::fs;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::io::Seek;

struct Store {
    file_store: FileStore,
    index: BTreeMap<Vec<u8>, CommandPosition>,
}

struct CommandPosition {
    pub file_id: usize,
    pub pos: usize,
    pub len: usize,
}

struct FileStore {
    current_write_log: File,
    read_logs: Vec<File>,
}

struct StoreIndex {
    map: BTreeMap<Vec<u8>, CommandPosition>,
}

impl Store {
    const DEFAULT_INITIAL_CAPACITY: usize = 128;

    /// the path must be a directory that all the data are stored in the directory
    pub fn open(path: PathBuf) -> KvdResult<Self> {
        // open the file
        // read the origin data and create in-memory index
        // return store
        let file_store = FileStore::open(path)?;
        let index = BTreeMap::new();
        Ok(Store {
            file_store,
            index,
        })
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KvdResult<()> {
        // write to the current file
        // update in-memory index
        // return result
        unimplemented!()
    }

    pub fn get(&self, key: Vec<u8>) -> KvdResult<Option<&Vec<u8>>> {
        // search in in-memory index, and read from file by index
        // return result
        unimplemented!()
    }

    pub fn del(&mut self, key: Vec<u8>) -> KvdResult<()> {
        // write to the current file
        // update in-memory index
        unimplemented!()
    }
}

impl FileStore {
    const DEFAULT_FILE_CAPACITY: u64 = 1024;

    pub fn open(path: PathBuf) -> KvdResult<FileStore> {
        fs::create_dir_all(&path)?;
        let sorted_file_number_list = Self::get_sorted_file_number_list(&path)?;

        let mut read_logs: Vec<File> = Vec::new();
        for file_num in sorted_file_number_list.iter() {
            let wal_path = Self::wal_path(&path, *file_num);
            let file = File::open(wal_path)?;
            read_logs.push(file);
        }

        // open the last file to write, if no file exists or the last file is full, create a new file
        let current_write_log = if sorted_file_number_list.is_empty() {
            Self::new_wal_file(Self::wal_path(&path, 0))?
        } else {
            let last_file_number = *sorted_file_number_list.last().unwrap();
            let wal_path = if Self::is_wal_file_full(read_logs.last().unwrap()) {
                Self::wal_path(&path, last_file_number + 1)
            } else {
                Self::wal_path(&path, last_file_number + 1)
            };
            Self::new_wal_file(wal_path)?
        };

        Ok(FileStore {
            read_logs,
            current_write_log,
        })
    }

    // important, focus on flat_map() and flatten()
    fn get_sorted_file_number_list(path: &Path) -> KvdResult<Vec<u64>> {
        let mut file_number_list: Vec<u64> = fs::read_dir(path)?
            .flat_map(|res| -> KvdResult<_> { Ok(res?.path()) })
            .filter(|path| Self::is_wal_file(path))
            .flat_map(|path| {
                path.file_name().and_then(OsStr::to_str).map(|s| s.trim_end_matches(".wal")).map(str::parse::<u64>)
            })
            .flatten()
            .collect();
        file_number_list.sort_unstable();
        Ok(file_number_list)
    }

    fn is_wal_file(path: &Path) -> bool {
        path.is_file() && path.starts_with("kvd_") && path.ends_with(".wal")
    }

    fn wal_path(path: &Path, file_number: u64) -> PathBuf {
        path.join(format!("kvd_{}.wal", file_number))
    }

    // make sure the file is valid
    fn is_wal_file_full(file: &File) -> bool {
        file.metadata().unwrap().len() >= Self::DEFAULT_FILE_CAPACITY
    }

    fn new_wal_file(path: PathBuf) -> KvdResult<File> {
        let result = OpenOptions::new().create(true).write(true).append(true).open(path)?;
        Ok(result)
    }
}

impl StoreIndex {
    pub fn new() -> StoreIndex {
        StoreIndex { map: BTreeMap::new() }
    }

    // load from pair iterator
    pub fn load() -> KvdResult<()> {
        unimplemented!()
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

    #[test]
    fn test_data_persistance() {
        // define test path
        let time = SystemTime::now();
        let time = time.duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path_str = format!("/tmp/kvd_store_persistence_{}.wal", time);
        let path = PathBuf::from(path_str);

        // define test data
        let key = Vec::from("key");
        let value = Vec::from("value");

        {
            let mut store = Store::open(path.clone()).unwrap();

            let result = store.get(key.clone());
            assert_eq!(Ok(None), result);

            let result = store.set(key.clone(), value.clone());
            assert_eq!(Ok(()), result);

            let result = store.get(key.clone());
            assert_eq!(Ok(Some(&value)), result);
            // drop the store automatically
        }

        // reopen the store and the data should be existed
        {
            let store = Store::open(path.clone()).unwrap();
            let result = store.get(key.clone());
            assert_eq!(Ok(Some(&value)), result);
        }
    }

    fn get_test_store() -> Store {
        let path = get_tmp_store_path();
        Store::open(path).unwrap()
    }

    fn get_tmp_store_path() -> PathBuf {
        let time = SystemTime::now();
        let time = time.duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path_str = format!("/tmp/kvd_store/", time);
        PathBuf::from(path_str)
    }
}
