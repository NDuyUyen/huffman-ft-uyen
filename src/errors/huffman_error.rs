use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct HuffmanError {
    pub msg: String,
    pub kind: HuffmanErrorKind,
}

impl HuffmanError {
    pub fn invalid_huffman_tree() -> Self {
        Self {
            msg: "The tree seems to be invalid.".to_string(),
            kind: HuffmanErrorKind::InvalidTree,
        }
    }
    pub fn not_found_in_tree() -> Self {
        Self {
            msg: "Cannot found item in Huffman tree.".to_string(),
            kind: HuffmanErrorKind::ItemNotFound,
        }
    }
    pub fn cannot_serialize_tree() -> Self {
        Self {
            msg: "Cannot serialize the Huffman tree".to_string(),
            kind: HuffmanErrorKind::InvalidTree,
        }
    }
    pub fn encoding_error() -> Self {
        Self {
            msg: "Cannot encode text".to_string(),
            kind: HuffmanErrorKind::EncodingError,
        }
    }
    pub fn decoding_error() -> Self {
        Self {
            msg: "Cannot decode text".to_string(),
            kind: HuffmanErrorKind::DecodingError,
        }
    }
}

impl fmt::Display for HuffmanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", &self.msg)
    }
}

#[derive(Debug, PartialEq)]
pub enum HuffmanErrorKind {
    InvalidTree,
    ItemNotFound,
    EncodingError,
    DecodingError,
}
