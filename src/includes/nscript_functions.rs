use base64::{encode, decode};

// Function to convert a string to base64
pub fn string_to_base64(string_in: &str) -> String {
    encode(string_in)
}

// Function to convert base64 to a string
pub fn base64_to_string(string_in: &str) -> String {
    match decode(string_in) {
        Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
        Err(_) => String::new(),
    }
}
