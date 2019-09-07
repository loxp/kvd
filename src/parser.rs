use super::command::Command;
use super::error::KvdError;

pub fn parse_command_from_one_line(line: &str) -> Result<Command, KvdError> {

}

#[cfg(test)]
mod tests {
    use super::parse_command_from_one_line;
    use super::Command;

    #[test]
    fn get_command_ok() {
        let ret = parse_command_from_one_line("GET key");
        if let Ok(Command::Get { key }) = ret {
            assert_eq!(key, Vec::from("key"));
        } else {
            assert!(false);
        }
    }
}
