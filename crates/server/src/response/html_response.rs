pub fn html_response(mut stream: TcpStream, html: &str) {
    let status_line = "HTTP/1.1 200 OK";

    let contents = html.clone();
    let contents_len = contents.len();

    let response = format!("{status_line}");

    println!("HTML response: {:#?}", response);

    stream.write_all(response.as_bytes()).unwrap();
}
