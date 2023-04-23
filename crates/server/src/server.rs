use crate::router::Router;
use crate::{connection::Connection, response::Responsable};

use std::{
    fmt::format,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct Server<T> {
    url: String,
    port: Option<i64>,
    router: Router<T>,
    listener: Option<TcpListener>,
    connections: Vec<Connection>,
}

impl<T> Server<T> {
    pub fn new(path: &str) -> Server<T> {
        Server {
            port: None,
            url: path.to_string(),
            listener: None,
            router: Router::new(),
            connections: Vec::new(),
        }
    }

    pub fn start_listening(&mut self, path: &str, port: i64) {
        let mut url = format!("{}:{}", path, port);
        let listener = TcpListener::bind(url.clone()).unwrap();

        println!("Listening for connections on port: {port}. {url}");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle(stream);
        }
    }

    fn handle(&mut self, stream: TcpStream) {
        let connection = Connection::new(stream);

        self.connections.push(connection);
        // self.router.handle(connection.p)
    }
}

pub fn start_listening(url: &str, port: i64) {
    let mut full_path = format!("{}:{}", url, port);

    let listener = TcpListener::bind(full_path.clone()).unwrap();

    println!("Listening for connections on port: {port}. {full_path}",);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let connection = Connection::new(stream);
    }

    println!("Server closed");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request line {request_line}");

    let html = fs::read_to_string("views/app.html").unwrap();

    if request_line == "GET / HTTP/1.1" {
        html_response(stream, html.as_str());
    } else {
        println!("Not found");

        let _split: Vec<_> = request_line.split_whitespace().collect();

        let formatted = format!("{request_line} Not found!");

        not_found(stream, formatted.as_str());
    }
}

fn not_found(mut stream: TcpStream, msg: &str) {
    let status_line: &str = "HTTP/1.1 404 NOT FOUND";
    let content_header = format!("Content-Length: {}", msg.len());

    let res = format!("{status_line}\r\n{content_header}\r\n\r\n{msg}");

    stream.write_all(res.as_bytes()).unwrap();
}

fn html_response(mut stream: TcpStream, html: &str) {
    let status_line: &str = "HTTP/1.1 200 OK";

    let body: &str = html.clone();

    let content_header = format!("Content-Length: {}", body.len());

    let response = format!("{status_line}\r\n{content_header}\r\n\r\n{body}");

    println!("HTML response: {:#?}", response);

    stream.write_all(response.as_bytes()).unwrap();
}
