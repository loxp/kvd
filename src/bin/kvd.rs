#[macro_use]
extern crate slog;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use clap::{App, Arg};
use config::Config;
use kvd::engine::bitcask::BitcaskEngine;
use kvd::engine::KvdEngine;
use kvd::model::KvdResult;
use kvd::server::Server;
use slog::Drain;
use slog_async;
use slog_scope;
use slog_scope::GlobalLoggerGuard;
use slog_term;
use std::fs::OpenOptions;
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

    let _log_guard = init_logger(&settings)?;

    let mut server = get_server(&settings)?;
    server.serve()
}

fn init_logger(settings: &Config) -> KvdResult<GlobalLoggerGuard> {
    let log_path = settings.get_str("log_path")?;
    let log_level = settings.get_str("log_level")?; // TODO: unused

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_path)?;

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());
    let guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();

    info!("standard logging redirected to slog");

    Ok(guard)
}

// TODO: 这里必须用impl KvdEngine, 否则编译报错. 原因?
fn get_server(config: &Config) -> KvdResult<Server<impl KvdEngine>> {
    let wal_dir = config.get_str("wal_dir")?;
    let engine = BitcaskEngine::open(PathBuf::from(wal_dir))?;
    let mut server = Server::new(engine)?;
    Ok(server)
}
