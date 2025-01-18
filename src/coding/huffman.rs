use super::super::errors::huffman_error::HuffmanError;
use super::super::models::huffman_tree::HuffmanTree;
use bitvec::prelude::*;
use std::collections::HashMap;

pub struct HuffmanEncodingResult {
    huffman_tree: HuffmanTree<char>,
    encoded_text: BitVec<u8, Msb0>,
}

impl HuffmanEncodingResult {
    pub fn new(huffman_tree: HuffmanTree<char>, encoded_text: BitVec<u8, Msb0>) -> Self {
        Self {
            huffman_tree: huffman_tree,
            encoded_text: encoded_text,
        }
    }

    pub fn get_encoded_text(&self) -> &BitVec<u8, Msb0> {
        &self.encoded_text
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
        let mut bin_str = String::new();

        match encoding_map {
            Ok(encoding_map) => {
                char_vec
                    .into_iter()
                    .try_for_each(|c| match encoding_map.get(&c) {
                        Some(encoded_c) => {
                            bin_str += encoded_c;
                            Ok(())
                        }
                        None => Err(HuffmanError::not_found_in_tree()),
                    })?;

                println!("bin_str: {}", bin_str);
                Ok(())
            }
            Err(e) => Err(e),
        }?;
        println!("bin_str: {}", bin_str);

        let num = isize::from_str_radix(&bin_str, 2).unwrap() as isize;

        println!("{:?}", &num);
        Err(HuffmanError::encoding_error())
        // let mut coding_map: HashMap<char, BitVec<u8, Msb0>> = HashMap::new();
        // let mut encoded_text: BitVec<u8, Msb0> = BitVec::new();

        // Ok(HuffmanEncodingResult::new(tree, encoded_text))
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

        // assert!(result.is_ok());
        // let huffman_encode = result.unwrap();
        // println!("Original text size: {}", (&text).get_heap_size());
        // println!(
        //     "Encoded text + codes size: {}",
        //     size_of_val(&huffman_encode)
        // );
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
