#[allow(dead_code)]
pub enum Status {
    Code200OK = 200,
    Code404NotFound = 404,
    Code405MethodNotAllowed = 405,
    Code500InternalServerError = 500,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        let result = match self {
            Status::Code200OK => "200 OK",
            Status::Code404NotFound => "404 Not Found",
            Status::Code405MethodNotAllowed => "405 Method Not Allowed",
            Status::Code500InternalServerError => "500 Internal Server Error",
        };

        String::from(result)
    }
}

pub fn response_string(status: Status, headers: Vec<String>, body: String) -> Vec<u8> {
    let body: Vec<u8> = body.as_bytes().to_vec();
    response_bytes(status, headers, body)
}

pub fn response_bytes(status: Status, mut headers: Vec<String>, mut body: Vec<u8>) -> Vec<u8> {
    let mut data = [
        format!("HTTP/1.1 {}", status.to_string()).as_str(),
        "Server: rust-http-server",
    ]
    .map(|line| line.to_string())
    .to_vec();

    data.append(&mut headers);
    data.push("\n".to_string());

    let mut data = data.join("\n").as_bytes().to_vec();
    data.append(&mut body);

    return data;
}
