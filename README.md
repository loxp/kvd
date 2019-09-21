[![Build Status](https://travis-ci.com/loxp/kvd.svg?branch=master)](https://travis-ci.com/loxp/kvd)

# Kvd

Kvd is a key-value database based on PingCAP Talent Plan.

## Table of Contents

- [Kvd](#kvd)
  - [Table of Contents](#table-of-contents)
  - [Background](#background)
  - [Install](#install)
  - [Usage](#usage)
  - [API](#api)
  - [Contributing](#contributing)

## Background

The project `Practical Networked Applications in Rust` in `PingCAP Talent Plan` (called `The Plan` later) is A training course about practical systems software construction in Rust.

The goal of kvd is same with `The Plan`: learning system programming in Rust. It is also divided into several parts:

- [x] Log-structured data store
- [ ] Synchronous networking
- [ ] Concurrency and parallelism
- [ ] Asynchronous programming
- [ ] Transaction (concurrency control and recovery)

## Install

```
git clone https://github.com/loxp/kvd
cargo build
```

## Usage

First clone and run kvd.

```
git clone https://github.com/loxp/kvd
cargo run -- --config=conf/default.yml
```

Now kvd supports command line user interface. It will be changed to client-server interface later, using Redis Protocol.

Set a key value pair.

```
set key value
"OK"
```

Get value by key.

```
get key
"value"
```

Delete key and its value.

```
del key
"OK"
```

There's no exit command now, use `ctrl+c` to quit.

## API

### SET

set key value

### GET

get key

### DEL

del key

## Contributing

Feel free to dive in! [Open an issue](https://github.com/RichardLitt/standard-readme/issues/new) or submit PRs.

Standard Readme follows the [Contributor Covenant](http://contributor-covenant.org/version/1/3/0/) Code of Conduct.