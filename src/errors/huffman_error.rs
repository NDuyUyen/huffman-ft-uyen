use std::{error::Error, fmt};

#[derive(Debug)]
pub struct HuffmanError {
    msg: String,
}

impl HuffmanError {
    pub fn encoding_error() -> Self {
        Self {
            msg: "Cannot encode text".to_string(),
        }
    }
}

impl fmt::Display for HuffmanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", &self.msg)
    }
}
