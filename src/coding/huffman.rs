use super::super::errors::huffman_error::HuffmanError;
use super::super::models::huffman_tree::HuffmanTree;
use bitvec::prelude::*;
use std::collections::HashMap;

pub struct HuffmanEncodingResult {
    codes: Vec<char>,
    encoded_text: BitVec<u8, Msb0>,
}

impl HuffmanEncodingResult {
    pub fn new(codes: Vec<char>, encoded_text: BitVec<u8, Msb0>) -> Self {
        Self {
            codes: codes,
            encoded_text: encoded_text,
        }
    }

    pub fn get_encoded_text(&self) -> &BitVec<u8, Msb0> {
        &self.encoded_text
    }

    pub fn get_codes(&self) -> &Vec<char> {
        &self.codes
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
        let tree = HuffmanTree::get_tree_as_vec(&char_vec);
        let mut coding_map: HashMap<char, BitVec<u8, Msb0>> = HashMap::new();
        let mut encoded_text: BitVec<u8, Msb0> = BitVec::new();

        char_vec.into_iter().for_each(|c| match coding_map.get(&c) {
            None => {
                let idx = tree.iter().position(|p| *p == c).unwrap();
                let mut data = vec![1u8; idx];
                // not correct, need to fix the logic getting encoded_char
                let mut encoded_char = BitVec::from_bitslice(data.view_bits_mut::<Msb0>());
                encoded_char.extend(if idx + 1 == tree.len() {
                    [1u8].iter()
                } else {
                    [0u8].iter()
                });
                encoded_text.extend(encoded_char.iter());
                coding_map.insert(c, encoded_char);
            }
            Some(code) => {
                encoded_text.extend(code);
            }
        });
        // println!("{:?}", &coding_map);
        Ok(HuffmanEncodingResult::new(tree, encoded_text))
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
        let huffman_encode = result.unwrap();
        println!("Original text size: {}", (&text).get_heap_size());
        println!(
            "Encoded text + codes size: {}",
            size_of_val(&huffman_encode)
        );
        // println!(
        //     "Encoded text + codes size: {:?}",
        //     huffman_encode.get_encoded_text()
        // );
    }

    fn test_decode() {
        let codes = vec![
            'W', 'c', 'd', 'r', 't', 'w', 'y', 'e', 'l', 'm', ' ', '!', 'o',
        ];
    }
}
