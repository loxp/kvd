use crate::engine::bitcask::BitcaskEngine;
use crate::engine::KvdEngine;
use crate::model;
use crate::model::KvdErrorKind::KeyNotFound;
use crate::model::{KvdError, KvdErrorKind, KvdResult};
use config::Config;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::str;
use std::{io, thread};

pub struct Server<T: KvdEngine> {
    engine: T,
    port: u16,
}

impl<T: KvdEngine> Server<T> {
    pub fn new(engine: T, port: u16) -> KvdResult<Server<T>> {
        let server = Server { engine, port };
        Ok(server)
    }

    pub fn serve(&mut self) -> KvdResult<()> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line?;
            self.handle_request(line)?;
        }
        Ok(())
    }

    pub fn serve_net(&mut self) -> KvdResult<()> {
        let listener = TcpListener::bind(("0.0.0.0", self.port))?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_conn(stream);
                }
                Err(e) => warn!("accept stream error: {:?}", e),
            }
        }

        Ok(())
    }

    fn handle_conn(&mut self, conn: TcpStream) -> KvdResult<()> {
        let mut reader = BufReader::new(&conn);
        let mut writer = BufWriter::new(&conn);
        for line in reader.lines() {
            let line = line?;
            let ret = self.handle_request(line)?;
            writer.write(ret.as_bytes())?;
            writer.flush();
        }
        Ok(())
    }

    fn handle_request(&mut self, line: String) -> KvdResult<String> {
        let request = model::parse_request_from_line(line)?;
        let result = self.dispatch_request(request);
        let ret = match result {
            Ok(r) => String::from_utf8(r).unwrap_or("not a utf-8 value".to_string()),
            Err(e) => format!("{:?}", e),
        };
        Ok(ret)
    }

    /// return a str in Vec for display
    fn dispatch_request(&mut self, request: Vec<Vec<u8>>) -> KvdResult<Vec<u8>> {
        let cmd = request
            .get(0)
            .ok_or(KvdError::from(KvdErrorKind::InvalidRequest))?;

        let result = match cmd.as_slice() {
            b"get" => self.handle_get(request).map(|r| r.unwrap_or(Vec::new())),
            b"set" => self.handle_set(request).and(Ok(Vec::new())),
            b"del" => self.handle_del(request).and(Ok(Vec::new())),
            _ => Err(KvdError::from(KvdErrorKind::InvalidRequest)),
        };

        result
    }

    // TODO: is it right to return a nil Vec when key is not found?
    fn handle_get(&mut self, request: Vec<Vec<u8>>) -> KvdResult<Option<Vec<u8>>> {
        if request.len() != 2 {
            return Err(KvdError::from(KvdErrorKind::InvalidRequest));
        }
        let result = self.engine.get(request.get(1).unwrap().clone())?;
        Ok(result)
    }

    fn handle_set(&mut self, request: Vec<Vec<u8>>) -> KvdResult<()> {
        if request.len() != 3 {
            return Err(KvdError::from(KvdErrorKind::InvalidRequest));
        }
        self.engine.set(
            request.get(1).unwrap().clone(),
            request.get(2).unwrap().clone(),
        )
    }

    fn handle_del(&mut self, request: Vec<Vec<u8>>) -> KvdResult<()> {
        if request.len() != 2 {
            return Err(KvdError::from(KvdErrorKind::InvalidRequest));
        }
        self.engine.del(request.get(1).unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::memory::MemoryEngine;

    #[test]
    fn test_new_server() {
        let engine = MemoryEngine::new();
        let server = Server::new(engine).unwrap();
    }
}
