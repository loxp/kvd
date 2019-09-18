use std::process::Command;
use assert_cmd::prelude::*;
use predicates::str::contains;

#[test]
fn test_kvd_no_args() {
    Command::cargo_bin("kvd").unwrap().assert().failure();
}

#[test]
fn test_kvd_version() {
    Command::cargo_bin("kvd").unwrap().arg("--version").assert().stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_kvd_create() {
    Command::cargo_bin("kvd").unwrap().arg("--config=conf/default.yaml").assert().success();
}

#[test]
fn test_kvd_get_set_del() {
    Command::cargo_bin("kvd").unwrap().arg("--config=conf/default.yaml").stdin();
    Command::stdin()
}

#[test]
fn test_kvd_quit() {}