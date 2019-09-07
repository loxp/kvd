// use commands::tokenizer::TokenizerError;
use std::error::Error;

#[derive(Debug, Fail)]
pub enum KvdError {
    #[fail(display = "parse command error: {}", err)]
    ErrParseCommand { err: String },
    #[fail(display = "invalid command : {}", cmd)]
    InvalidCommand { cmd: String },
    #[fail(display = "invalid arg number: {}", num)]
    InvalidArgNumber { num: i32 },
}

// impl From<TokenizerError> for KvdError {
//     fn from(err: TokenizerError) -> Self {
//         Self::ErrParseCommand{err: err.description().to_string()}
//     }
// }