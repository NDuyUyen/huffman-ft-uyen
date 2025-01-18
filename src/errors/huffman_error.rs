use std::{error::Error, fmt};

#[derive(Debug)]
pub struct HuffmanError {
    msg: String,
}

impl HuffmanError {
    pub fn invalid_huffman_tree() -> Self {
        Self {
            msg: "The tree seems to be invalid.".to_string(),
        }
    }
    pub fn not_found_in_tree() -> Self {
        Self {
            msg: "Cannot found item in Huffman tree.".to_string(),
        }
    }
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
