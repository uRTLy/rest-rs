use std::net::TcpStream;

const NEW_LINE: &str = "\r\n\r\n";

fn http_response(status: &str, headers: &str, body: &str) {
    return format!("{status}{}{headers}{}{body}", NEW_LINE, NEW_LINE).as_bytes();
}
