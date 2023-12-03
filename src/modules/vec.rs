pub fn slice_to_bytes_vec(slice: Vec<String>) -> Vec<u8> {
    return slice.join("\n").as_bytes().to_owned();
}

pub fn slice_to_vec(slice: &[&str]) -> Vec<String> {
    slice.iter().map(|line| line.to_string()).collect()
}
