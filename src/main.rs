extern crate redis;
use anyhow;

use std::io::prelude::*;
use redis::Commands;
use std::str;
use std::net::{self, TcpStream, ToSocketAddrs};

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

struct Client {
    io: TcpStream
}

impl Client {
    fn new(tcp: TcpStream) -> Self {
        Client {io:tcp}
    }

    fn get(&mut self, key:&str) -> Result<String>{
        let query = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
        self.io.write(query.as_bytes())?;
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..])?;
        Ok(str::from_utf8(&buffer)?.to_string())
    }

    fn set(&mut self, key:&str, value: &str) -> Result<String>{
        let query = format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n", key.len(), key, value.len(), value);
        self.io.write(query.as_bytes())?;
        let mut buffer = [0; 512];
        self.io.read(&mut buffer[..])?;
        Ok(str::from_utf8(&buffer)?.to_string())
    }
}


fn main() -> Result<()> {
    let tcp = TcpStream::connect("127.0.0.1:6379")?;
    let mut clinet = Client {io: tcp};
    let res = clinet.set("my_key", &999.to_string())?;
    let res = clinet.get("my_key")?;

    println!("{}", res);
    Ok(())
}
