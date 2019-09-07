pub enum Command {
    Get { key: Vec<u8> },
    Set { key: Vec<u8>, value: Vec<u8> },
    Del { key: Vec<u8> },
}
