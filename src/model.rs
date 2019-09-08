use failure::_core::convert::TryFrom;

type Command = Vec<Vec<u8>>;

#[derive(Debug, Fail, PartialEq)]
pub enum KvdError {
    #[fail(display = "key not found")]
    KeyNotFound,
}

pub type KvdResult<T> = Result<T, KvdError>;

pub fn parse_command_from_string(cmd: String) -> KvdResult<Command> {
    let tokens = cmd
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
            let cmd = parse_command_from_string(testcase.input.to_string()).unwrap();
            for j in 0..testcase.expect.len() {
                let a = testcase.expect[j].as_bytes();
                let b = cmd.get(j).unwrap().as_slice();
                assert_eq!(a, b);
            }
        }
    }
}
