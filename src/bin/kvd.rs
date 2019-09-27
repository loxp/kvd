use clap::{App, Arg};
use config::Config;
use kvd::engine::bitcask::BitcaskEngine;
use kvd::engine::KvdEngine;
use kvd::model::{KvdError, KvdResult};
use kvd::server::Server;
use std::path::PathBuf;

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

    let config_path = matches.value_of("config").unwrap();
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(config_path))?;

    let mut server = get_server(settings)?;
    server.serve()
}

// TODO: 这里必须用impl KvdEngine, 否则编译报错. 原因?
fn get_server(config: Config) -> KvdResult<Server<impl KvdEngine>> {
    let wal_dir = config.get_str("wal_dir")?;
    let engine = BitcaskEngine::open(PathBuf::from(wal_dir))?;
    let mut server = Server::new(engine)?;
    Ok(server)
}
