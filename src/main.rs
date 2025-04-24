use std::io;

use rust_http_server::listen;

mod modules;

fn main() -> io::Result<()> {
    listen("0.0.0.0:8080")
}
