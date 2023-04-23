mod connection;
mod headers;
mod http;
mod request;
mod response;
mod router;
mod server;

mod test_utils;

fn main() {
    server::start_listening("127.0.0.1", 7878);
}
