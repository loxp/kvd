use crate::model::{KvdError, KvdResult};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

const DEFAULT_FILE_CAPACITY: u64 = 1024;

struct Store {
    file_store: FileStore,
    index: BTreeMap<Vec<u8>, CommandPosition>,
}

struct CommandPosition {
    pub file_number: u64,
    pub pos: usize,
    pub len: usize,
}

struct FileStore {
    current_file_num: u64,
    current_write_log: WalWriter<File>,
    read_logs: Vec<WalReader<File>>,
}

struct WalWriter<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

struct WalReader<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
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
        Ok(Store { file_store, index })
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
    pub fn open(path: PathBuf) -> KvdResult<FileStore> {
        fs::create_dir_all(&path)?;

        let mut sorted_file_number_list = Self::get_sorted_file_number_list(&path)?;
        if sorted_file_number_list.is_empty() {
            let mut readers: Vec<WalReader<File>> = Vec::new();
            let writer = Self::build_wal_writer(&path, 0)?;
            Ok(FileStore {
                current_file_num: 0,
                current_write_log: writer,
                read_logs: readers,
            })
        } else {
            // take out the last file, and put all other files into reader list
            let last_file_num = sorted_file_number_list.pop().unwrap();
            let mut readers: Vec<WalReader<File>> = Vec::new();
            for file_num in sorted_file_number_list.iter() {
                let wal_path = Self::wal_path(&path, *file_num);
                let read_wal = File::open(wal_path)?;
                let reader = WalReader::new(read_wal)?;
                readers.push(reader);
            }

            let wal_path = Self::wal_path(&path, last_file_num);
            let read_wal = File::open(wal_path)?;
            let reader = WalReader::new(read_wal)?;
            readers.push(reader);

            let writer = Self::build_wal_writer(&path, last_file_num)?;
            Ok(FileStore {
                current_file_num: last_file_num,
                current_write_log: writer,
                read_logs: readers,
            })
        }
    }

    fn build_wal_writer(path: &Path, file_num: u64) -> KvdResult<WalWriter<File>> {
        let path = Self::wal_path(&path, file_num);
        let file = Self::new_wal_file(path)?;
        let writer = WalWriter::new(file)?;
        Ok(writer)
    }

    pub fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) -> KvdResult<CommandPosition> {
        unimplemented!()
        // serialize the key value pair
        // write to file
        // get the file number, offset and size
    }

    // important, focus on flat_map() and flatten()
    fn get_sorted_file_number_list(path: &Path) -> KvdResult<Vec<u64>> {
        let mut file_number_list: Vec<u64> = fs::read_dir(path)?
            .flat_map(|res| -> KvdResult<_> { Ok(res?.path()) })
            .filter(|path| Self::is_wal_file(path))
            .flat_map(|path| {
                path.file_name()
                    .and_then(OsStr::to_str)
                    .map(|s| s.trim_end_matches(".wal"))
                    .map(str::parse::<u64>)
            })
            .flatten()
            .collect();
        file_number_list.sort_unstable();
        Ok(file_number_list)
    }

    fn change_to_new_wal(&mut self) -> KvdResult<()> {
        unimplemented!()
    }

    fn is_wal_file(path: &Path) -> bool {
        path.is_file() && path.starts_with("kvd_") && path.ends_with(".wal")
    }

    fn wal_path(path: &Path, file_number: u64) -> PathBuf {
        path.join(format!("kvd_{}.wal", file_number))
    }

    fn new_wal_file(path: PathBuf) -> KvdResult<File> {
        let result = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)?;
        Ok(result)
    }
}

impl StoreIndex {
    pub fn new() -> StoreIndex {
        StoreIndex {
            map: BTreeMap::new(),
        }
    }

    // load from pair iterator
    pub fn load() -> KvdResult<()> {
        unimplemented!()
    }
}

impl<W: Write + Seek> WalWriter<W> {
    fn new(mut inner: W) -> KvdResult<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(WalWriter {
            writer: BufWriter::new(inner),
            pos,
        })
    }

    fn is_full(&self) -> bool {
        self.pos >= DEFAULT_FILE_CAPACITY
    }
}

impl<W: Write + Seek> Write for WalWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let length = self.writer.write(buf)?;
        self.pos += length as u64;
        Ok(length)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for WalWriter<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

impl<R: Read + Seek> WalReader<R> {
    fn new(mut inner: R) -> KvdResult<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(WalReader {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for WalReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for WalReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
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
