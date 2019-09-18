use config::ConfigError;
use failure::_core::fmt::Display;
use failure::_core::str::Utf8Error;
use failure::{Backtrace, Context, Fail};
use std::fmt::{Error, Formatter};
use std::io;
use std::string::FromUtf8Error;

type Request = Vec<Vec<u8>>;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum KvdErrorKind {
    #[fail(display = "key not found")]
    KeyNotFound,
    #[fail(display = "invalid request")]
    InvalidRequest,
    #[fail(display = "invalid command")]
    InvalidCommand,
    #[fail(display = "path is not directory")]
    PathIsNotDirectory,
    #[fail(display = "io error")]
    Io,
    #[fail(display = "serde error")]
    Serde,
    #[fail(display = "file not found")]
    FileNotFound,
    #[fail(display = "config error")]
    Config,
    #[fail(display = "string convert error")]
    StringConvertError,
}

#[derive(Debug)]
pub struct KvdError {
    ctx: Context<KvdErrorKind>,
}

impl KvdError {
    pub fn kind(&self) -> KvdErrorKind {
        *self.ctx.get_context()
    }
}

/// TODO: is it right?
impl PartialEq for KvdError {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}

impl Fail for KvdError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl Display for KvdError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.ctx, f)
    }
}

impl From<KvdErrorKind> for KvdError {
    fn from(kind: KvdErrorKind) -> Self {
        KvdError {
            ctx: Context::new(kind),
        }
    }
}

impl From<Context<KvdErrorKind>> for KvdError {
    fn from(ctx: Context<KvdErrorKind>) -> Self {
        KvdError { ctx }
    }
}

impl From<io::Error> for KvdError {
    fn from(e: io::Error) -> Self {
        KvdError {
            ctx: Context::new(KvdErrorKind::Io),
        }
    }
}

impl From<serde_json::error::Error> for KvdError {
    fn from(e: serde_json::error::Error) -> Self {
        KvdError {
            ctx: Context::new(KvdErrorKind::Serde),
        }
    }
}

impl From<ConfigError> for KvdError {
    fn from(e: ConfigError) -> Self {
        KvdError {
            ctx: Context::new(KvdErrorKind::Config),
        }
    }
}

impl From<Utf8Error> for KvdError {
    fn from(e: Utf8Error) -> Self {
        KvdError {
            ctx: Context::new(KvdErrorKind::StringConvertError),
        }
    }
}

pub type KvdResult<T> = Result<T, KvdError>;

pub fn parse_request_from_line(line: String) -> KvdResult<Request> {
    let tokens = line
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| Vec::from(s))
        .collect();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_from_string() {
        struct Testcase {
            input: &'static str,
            expect: &'static [&'static str],
        }

        let mut testcases = Vec::new();
        testcases.push(Testcase {
            input: "get key",
            expect: &["get", "key"],
        });
        testcases.push(Testcase {
            input: "get  key ",
            expect: &["get", "key"],
        });
        testcases.push(Testcase {
            input: "get  key11 ",
            expect: &["get", "key11"],
        });
        testcases.push(Testcase {
            input: "set  key11 hello",
            expect: &["set", "key11", "hello"],
        });

        for i in 0..testcases.len() {
            let testcase = testcases.get(i).unwrap();
            let cmd = parse_request_from_line(testcase.input.to_string()).unwrap();
            for j in 0..testcase.expect.len() {
                let a = testcase.expect[j].as_bytes();
                let b = cmd.get(j).unwrap().as_slice();
                assert_eq!(a, b);
            }
        }
    }
}
