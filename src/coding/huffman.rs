use super::super::errors::huffman_error::HuffmanError;
use super::super::models::huffman_tree::HuffmanTree;
use bitvec::prelude::*;
use std::collections::HashMap;

pub struct HuffmanEncodingResult {
    codes: Vec<char>,
    encoded_text: BitVec,
}

impl HuffmanEncodingResult {
    pub fn new(codes: Vec<char>, encoded_text: BitVec) -> Self {
        Self {
            codes: codes,
            encoded_text: encoded_text,
        }
    }

    pub fn get_encoded_text(&self) -> &BitVec {
        &self.encoded_text
    }

    pub fn get_codes(&self) -> &Vec<char> {
        &self.codes
    }
}

trait HuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, HuffmanError>;
    fn decode() -> Result<HuffmanTree<char>, HuffmanError>;
}
pub struct StandardHuffmanCoding {}

impl HuffmanCoding for StandardHuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, HuffmanError> {
        let char_vec: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::get_tree_as_vec(&char_vec);
        let mut coding_map: HashMap<char, BitVec> = HashMap::new();
        let mut encoded_text: BitVec = BitVec::new();

        let encode_result = char_vec
            .into_iter()
            .try_for_each(|c| -> Result<(), HuffmanError> {
                if coding_map.contains_key(&c) {
                    encoded_text.extend(coding_map.get(&c).unwrap());
                    Ok(())
                } else {
                    let idx = tree.iter().position(|p| *p == c).unwrap();
                    let base = BitVec::new();
                    BitVec::from_element(idx, true);
                    let result = match base {
                        Ok(base) => {
                            let encoded_char = if idx + 1 == tree.len() {
                                base + "1"
                            } else {
                                base + "0"
                            };
                            encoded_text += &encoded_char;
                            coding_map.insert(c, encoded_char);
                            Ok(())
                        }
                        Err(_) => Err(HuffmanError::encoding_error()),
                    };
                    return result;
                }
            });

        println!("{:?}", coding_map);
        match encode_result {
            Ok(()) => Ok(HuffmanEncodingResult::new(tree, encoded_text)),
            Err(e) => Err(e),
        }
    }

    fn decode() -> Result<HuffmanTree<char>, HuffmanError> {
        Err(HuffmanError::encoding_error())
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_encode() {
        let text = "Nguyen Duy Uyen iu chi Beo".to_string();
        let result = StandardHuffmanCoding::encode(&text);

        assert!(result.is_ok());
        let huffman_encode = result.unwrap();
        println!("{}", huffman_encode.get_encoded_text());
        println!("{:?}", huffman_encode.get_codes());
        // assert!(encoded.is_ok());
    }
}
