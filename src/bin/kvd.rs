use clap::{App, Arg};
use kvd::model::{KvdError, KvdResult};
use kvd::server::Server;

fn main() -> KvdResult<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap();
    let mut server = Server::new(config)?;
    server.serve();
    Ok(())
}
