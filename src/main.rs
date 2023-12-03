mod modules;

use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::modules::{
    file::{get_mime, read_file_vec},
    http::{http_200, HTTP_404, HTTP_405},
    vec::{slice_to_bytes_vec, slice_to_vec},
};

fn write_response(stream: &mut TcpStream, payload: Vec<String>) {
    match stream.write(slice_to_bytes_vec(payload).as_slice()) {
        Ok(_) => {}
        Err(data) => {
            println!("{:#?}", data);
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);

    let http_request: Vec<String> = reader
        .lines()
        .map(|result| result.unwrap_or_default())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let line1: Vec<&str> = http_request[0].split(" ").into_iter().collect();
    let (http_method, http_path, _http_version) = (line1[0], line1[1], line1[2]);
    if http_method != "GET" {
        write_response(&mut stream, slice_to_vec(&HTTP_405));
        return Ok(());
    }

    match read_file_vec(format!("./public{}", http_path).as_str()) {
        Ok(data) => {
            let payload = http_200(
                get_mime(http_path.split(".").last().unwrap_or_default()),
                vec![data.lines().collect()],
            );
            write_response(&mut stream, payload);
        }
        Err(_) => {
            write_response(&mut stream, slice_to_vec(&HTTP_404));
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
