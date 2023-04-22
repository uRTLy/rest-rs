use std::fs;

mod server;

fn main() {
    let html = fs::read_to_string("views/app.html").unwrap();

    println!("{html}");
    server::start_listening("127.0.0.1", 7878);
}
