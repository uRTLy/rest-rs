use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn start_listening(url: &str, port: i64) {
    let mut full_path = format!("{}:{}", url, port);

    let listener = TcpListener::bind(full_path.clone()).unwrap();

    println!("Listening for connections on port: {port}. {full_path}",);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    println!("Server closed");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let html = fs::read_to_string("app.html").unwrap();

    html_response(stream, html.as_str());
}

fn text_response(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    println!("Response: {:#?}", response);

    stream.write_all(response.as_bytes()).unwrap();
}

fn html_response(mut stream: TcpStream, html: &str) {
    let status_line: &str = "HTTP/1.1 200 OK";

    let body: &str = html.clone();

    let content_header = format!("Content-Length: {}", body.len());

    let response = format!("{status_line}\r\n{content_header}\r\n\r\n{body}");

    println!("HTML response: {:#?}", response);

    stream.write_all(response.as_bytes()).unwrap();
}
