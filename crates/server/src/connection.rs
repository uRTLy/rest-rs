use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::headers::Header;
use crate::http::{HttpMethod, HttpStatus};

pub struct Connection {
    path: String,
    stream: TcpStream,
    method: HttpMethod,
    status: HttpStatus,
    headers: Vec<Header>,
}

impl Connection {
    pub fn new(mut stream: TcpStream) -> Connection {
        let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

        let lines: Vec<_> = buf_reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let path: String = String::from("/");
        let headers: Vec<Header> = vec![];
        let status: HttpStatus = HttpStatus::OK;
        let method: HttpMethod = HttpMethod::GET;

        Connection {
            path,
            stream,
            headers,
            status,
            method,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Connection;

    #[test]
    fn new() {
        // let r = Connection::new();
    }
}
