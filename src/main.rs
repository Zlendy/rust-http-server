use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);

    let http_request: Vec<String> = reader
        .lines()
        .map(|result| result.unwrap_or_default())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let http_response: Vec<String> = [
        // TODO: Dynamic response
        "HTTP/1.1 200 OK",
        "Server: Rust",
        "Content-Type: text/plain",
        "",
        "Hello, world!",
    ]
    .map(|line| String::from(line))
    .to_vec();

    match stream.write(http_response.join("\n").as_bytes()) {
        Ok(_) => {}
        Err(data) => {
            println!("{:#?}", data);
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:8080";
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
