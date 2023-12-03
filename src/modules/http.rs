pub const HTTP_404: [&str; 5] = [
    "HTTP/1.1 404 Not found",
    "Server: Rust",
    "Content-Type: text/plain",
    "",
    "The requested resource was not found on this server",
];

pub const HTTP_405: [&str; 5] = [
    "HTTP/1.1 405 Method Not Allowed",
    "Server: Rust",
    "Content-Type: text/plain",
    "",
    "This server only accepts GET requests",
];

pub fn http_200(content_type: &str, mut data: Vec<String>) -> Vec<String> {
    let mut header = [
        "HTTP/1.1 200 OK",
        "Server: Rust",
        format!("Content-Type: {}", content_type).as_str(),
        "",
    ]
    .map(|line| line.to_string())
    .to_vec();

    header.append(&mut data);

    return header;
}
