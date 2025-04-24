mod modules;

use crate::modules::stream::handle_client;
use std::{io, net::TcpListener};

pub fn listen(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    println!("Server listening on {}", address);

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // do something with the TcpStream
                let _ = handle_client(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                // wait_for_fd();
                continue;
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }

    Ok(())
}
