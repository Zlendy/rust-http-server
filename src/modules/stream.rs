use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::modules::{
    file::{get_mime, read_file_vec},
    http::{response_bytes, response_string, Status},
};

fn write_response(stream: &mut TcpStream, payload: Vec<u8>) {
    match stream.write(payload.as_slice()) {
        Ok(_) => {}
        Err(data) => {
            println!("{:#?}", data);
        }
    }
}

pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
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
        write_response(
            &mut stream,
            response_string(
                Status::Code405MethodNotAllowed,
                vec![],
                "This server only accepts GET requests".to_string(),
            ),
        );
        return Ok(());
    }

    match read_file_vec(format!("./public{}", http_path).as_str()) {
        Ok(data) => {
            write_response(
                &mut stream,
                response_bytes(
                    Status::Code200OK,
                    vec![format!(
                        "Content-Type: {}",
                        get_mime(http_path.split(".").last().unwrap_or_default())
                    )],
                    data,
                ),
            );
        }
        Err(_) => {
            write_response(
                &mut stream,
                response_string(
                    Status::Code404NotFound,
                    vec![],
                    "The requested resource was not found on this server".to_string(),
                ),
            );
        }
    }

    Ok(())
}
