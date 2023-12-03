use std::{fs, path::Path};

pub fn get_mime(extension: &str) -> &str {
    match extension {
        "html" => "text/html",
        "htm" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        _ => "text/plain",
    }
}

pub fn read_file_vec(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let data = fs::read_to_string(path)?;
    Ok(data)
}
