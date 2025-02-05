use crate::{
    coding::huffman::{HuffmanCoding, StandardHuffmanCoding},
    errors::compression_error::CompressionError,
};

pub fn compress(text: &String) -> Result<String, CompressionError> {
    match <StandardHuffmanCoding as HuffmanCoding>::encode(&text) {
        Ok(encoding_result) => Ok(encoding_result.serialize()),
        Err(e) => Err(CompressionError::cannot_compress_text(e.msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let text = "Huffman-ft-uyen".to_string();
        let result = compress(&text);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            "1-001f01n1-0001y1t1u001a1H01m1e-m\u{7}1\u{19}\u{17}1t"
        );
    }
}
