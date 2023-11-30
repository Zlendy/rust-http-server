use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut string = String::new();
    let payload = stream.read_to_string(&mut string);

    match payload {
        Ok(_) => {
            println!("Incoming: {}", string);
        }
        Err(data) => {
            println!("ERROR: {}", data)
        }
    }
}

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
