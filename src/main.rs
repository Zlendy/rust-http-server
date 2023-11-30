use std::{
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: TcpStream) {
    let stream = BufReader::new(stream);
    for payload in stream.lines() {
        match payload {
            Ok(data) => {
                println!("Incoming: {}", data);
            }
            Err(data) => {
                println!("ERROR: {}", data)
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
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
                handle_client(s);
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
