use super::super::errors::huffman_error::HuffmanError;
use super::super::models::huffman_tree::HuffmanTree;
use bitvec::prelude::*;
use std::collections::HashMap;

pub struct HuffmanEncodingResult {
    huffman_tree: HuffmanTree<char>,
    encoded_vec: Vec<bool>,
}

impl HuffmanEncodingResult {
    pub fn new(huffman_tree: HuffmanTree<char>, encoded_vec: Vec<bool>) -> Self {
        Self {
            huffman_tree: huffman_tree,
            encoded_vec: encoded_vec,
        }
    }

    pub fn get_encoded_vec(&self) -> &Vec<bool> {
        &self.encoded_vec
    }

    pub fn get_huffman_tree(&self) -> &HuffmanTree<char> {
        &self.huffman_tree
    }
}

pub struct HuffmanDecodingResult {
    decoded_text: String,
}

impl HuffmanDecodingResult {
    pub fn new(decoded_text: String) -> Self {
        Self {
            decoded_text: decoded_text,
        }
    }

    pub fn get_decoded_text(&self) -> &String {
        &self.decoded_text
    }
}

trait HuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, HuffmanError>;
    fn decode(
        codes: &Vec<char>,
        encoded_text: &BitVec<u8, Msb0>,
    ) -> Result<HuffmanDecodingResult, HuffmanError>;
}
pub struct StandardHuffmanCoding {}

impl HuffmanCoding for StandardHuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, HuffmanError> {
        let char_vec: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::from(&char_vec);
        let encoding_map = tree.get_encoding_map();
        let mut encoded_vec: Vec<bool> = Vec::new();

        match encoding_map {
            Ok(encoding_map) => {
                char_vec
                    .into_iter()
                    .try_for_each(|c| match encoding_map.get(&c) {
                        Some(encoded_c) => {
                            encoded_vec.append(&mut encoded_c.clone());
                            Ok(())
                        }
                        None => Err(HuffmanError::not_found_in_tree()),
                    })?;
                Ok(())
            }
            Err(e) => Err(e),
        }?;

        Ok(HuffmanEncodingResult::new(tree, encoded_vec))
    }

    fn decode(
        codes: &Vec<char>,
        encoded_text: &BitVec<u8, Msb0>,
    ) -> Result<HuffmanDecodingResult, HuffmanError> {
        let mut iter = encoded_text.iter();
        while iter.next().is_some() {}

        loop {
            let f = iter.next();
            match f {
                Some(s) => {
                    println!("{}", s);
                }
                None => {}
            }
            if iter.next().is_none() {
                break;
            }
        }
        Err(HuffmanError::encoding_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use get_size::GetSize;
    use std::mem::size_of_val;

    #[test]
    fn test_encode() {
        let text = "Welcome to my world!!!".to_string();
        let result = StandardHuffmanCoding::encode(&text);

        assert!(result.is_ok());
        let result_encoded_vec = result.unwrap().encoded_vec;
        let l = false;
        let r = true;
        let expected_encoded_vec = vec![
            l, r, r, l, l, r, l, r, r, r, r, l, r, r, r, r, r, l, l, r, r, r, l, l, r, l, r, r, l,
            l, l, l, l, r, l, l, r, r, l, r, r, r, l, l, l, r, l, r, r, l, l, l, r, r, r, l, l, l,
            l, l, r, r, r, r, r, l, r, r, r, l, r, l, r, r, l, r, r, l, r,
        ];
        assert_eq!(result_encoded_vec, expected_encoded_vec);
    }

    fn test_decode() {
        let codes = vec![
            'W', 'c', 'd', 'r', 't', 'w', 'y', 'e', 'l', 'm', ' ', '!', 'o',
        ];
    }
}
