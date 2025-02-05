use std::fmt;

#[derive(Debug, PartialEq)]
pub struct CompressionError {
    pub msg: String,
}

impl CompressionError {
    pub fn cannot_compress_text(detail: String) -> Self {
        let msg: String = format!("Cannot compress text input: {}", detail);
        Self { msg: msg }
    }
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", &self.msg)
    }
}
