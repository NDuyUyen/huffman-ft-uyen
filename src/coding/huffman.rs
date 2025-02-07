use crate::errors::huffman_error::HuffmanError;
use crate::models::huffman_tree::HuffmanTree;
use crate::utils::type_converting;

#[derive(Clone)]
pub struct HuffmanEncodingResult {
    huffman_tree: HuffmanTree<char>,
    encoded_vec: Vec<bool>,
}

impl HuffmanEncodingResult {
    const ASCII_FORM: usize = 7;
    const DELIMITER: &str = "-";

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

    pub fn serialize(&self) -> String {
        let mut encoded_vec = self.encoded_vec.clone();
        let filled_bits = Self::fill_bits(&mut encoded_vec);
        let tree_str = self.huffman_tree.serialize();
        let tree_size = tree_str.len();
        let encoded_vec_str =
            type_converting::vec_bool_to_string(&mut encoded_vec, Self::ASCII_FORM);

        let components_list = vec![
            filled_bits.to_string(),
            tree_size.to_string(),
            tree_str + &encoded_vec_str,
        ];

        return components_list.join(Self::DELIMITER);
    }

    pub fn deserialize(input: String) -> Result<HuffmanEncodingResult, HuffmanError> {
        let error = HuffmanError::cannot_deserialize_tree();

        match input.split_once(HuffmanEncodingResult::DELIMITER) {
            Some((filled_bits_str, remaining)) => {
                let filled_bits = type_converting::str_to_usize(filled_bits_str)
                    .map_err(|_| error.clone())
                    .unwrap();

                match remaining.split_once(HuffmanEncodingResult::DELIMITER) {
                    Some((tree_size_str, remaining)) => {
                        let tree_size = type_converting::str_to_usize(tree_size_str)
                            .map_err(|_| error.clone())
                            .unwrap();

                        let (tree_str, encoded_str) = remaining.split_at(tree_size as usize);
                        let huffman_tree = HuffmanTree::deserialize(tree_str.to_string())
                            .map_err(|_| error.clone())
                            .unwrap();

                        let encoded_vec = HuffmanEncodingResult::calculate_encoded_vec(
                            encoded_str,
                            filled_bits as usize,
                        );

                        Ok(HuffmanEncodingResult::new(huffman_tree, encoded_vec))
                    }
                    None => Err(error),
                }
            }
            None => Err(error),
        }
    }

    fn fill_bits(bits: &mut Vec<bool>) -> usize {
        let bits_should_fill =
            HuffmanEncodingResult::ASCII_FORM - (bits.len() % HuffmanEncodingResult::ASCII_FORM);
        for _ in 0..bits_should_fill {
            bits.push(false);
        }

        bits_should_fill
    }

    fn calculate_encoded_vec(input: &str, filled_bits: usize) -> Vec<bool> {
        let mut full_vec = type_converting::string_to_vec_bool(input);
        let new_len = full_vec.len() - filled_bits;

        full_vec.split_off(new_len)
    }
}

#[derive(Debug, PartialEq)]
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

pub trait HuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, HuffmanError>;
    fn decode(
        huffman_tree: HuffmanTree<char>,
        encoded_vec: Vec<bool>,
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
        huffman_tree: HuffmanTree<char>,
        encoded_vec: Vec<bool>,
    ) -> Result<HuffmanDecodingResult, HuffmanError> {
        let mut iter: std::slice::Iter<'_, bool> = encoded_vec.iter();
        let mut decoded_text = String::new();

        while iter.len() > 0 {
            match huffman_tree.decode_by_path(&mut iter) {
                Ok(next_char) => decoded_text.push(next_char),
                Err(e) => return Err(e),
            }
        }

        Ok(HuffmanDecodingResult::new(decoded_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_successful() {
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

    #[test]
    fn test_decode_successful() {
        let text = "Welcome to my world!!!".to_string();
        let l = false;
        let r = true;
        let encoded_result = StandardHuffmanCoding::encode(&text).unwrap();
        let tree = encoded_result.huffman_tree.clone();
        let encoded_vec = encoded_result.encoded_vec.clone();
        let decoded_result = StandardHuffmanCoding::decode(tree, encoded_vec);

        assert!(decoded_result.is_ok());
        assert_eq!(decoded_result.unwrap().decoded_text, text);

        let input = "Welcome to my world".to_string();
        let encoded_vec = vec![
            l, r, r, l, l, r, l, r, r, r, r, l, r, r, r, r, r, l, l, r, r, r, l, l, r, l, r, r, l,
            l, l, l, l, r, l, l, r, r, l, r, r, r, l, l, l, r, l, r, r, l, l, l, r, r, r, l, l, l,
            l, l, r, r, r, r, r, l, r, r, r, l,
        ];
        let tree = encoded_result.huffman_tree.clone();
        let decoded_result = StandardHuffmanCoding::decode(tree, encoded_vec);

        assert_eq!(decoded_result.unwrap().decoded_text, input);

        let input = "Welcome to my world!!!!!!".to_string();
        let encoded_vec = vec![
            l, r, r, l, l, r, l, r, r, r, r, l, r, r, r, r, r, l, l, r, r, r, l, l, r, l, r, r, l,
            l, l, l, l, r, l, l, r, r, l, r, r, r, l, l, l, r, l, r, r, l, l, l, r, r, r, l, l, l,
            l, l, r, r, r, r, r, l, r, r, r, l, r, l, r, r, l, r, r, l, r, r, l, r, r, l, r, r, l,
            r,
        ];
        let tree = encoded_result.huffman_tree.clone();
        let decoded_result = StandardHuffmanCoding::decode(tree, encoded_vec);

        assert_eq!(decoded_result.unwrap().decoded_text, input);

        let input = String::new();
        let a_part_encoded_vec: Vec<bool> = vec![];
        let tree = encoded_result.huffman_tree.clone();
        let decoded_result = StandardHuffmanCoding::decode(tree, a_part_encoded_vec);

        assert_eq!(decoded_result.unwrap().decoded_text, input);
    }

    #[test]
    fn test_decode_failed() {
        let text = "Welcome to my world!!!".to_string();
        let l = false;
        let r = true;
        let encoded_result = StandardHuffmanCoding::encode(&text).unwrap();

        let tree = encoded_result.huffman_tree.clone();
        let undecodable_vec = vec![
            l, r, r, l, l, r, l, r, r, r, r, l, r, r, r, r, r, l, l, r, r, r, l, l, r, l, r, r, l,
            l, l, l, l, r, l, l, r, r, l, r, r, r, l, l, l, r, l, r, r, l, l, l, r, r, r, l, l, l,
            l, l, r, r, r, r, r, l, r, r, r, l, r,
        ];
        let decoded_result = StandardHuffmanCoding::decode(tree, undecodable_vec);

        assert!(decoded_result.is_err());
        assert_eq!(decoded_result, Err(HuffmanError::decoding_error()));

        let tree = encoded_result.huffman_tree.clone();
        let undecodable_vec = vec![l];
        let decoded_result = StandardHuffmanCoding::decode(tree, undecodable_vec);

        assert!(decoded_result.is_err());
        assert_eq!(decoded_result, Err(HuffmanError::decoding_error()));
    }

    #[test]
    fn test_serialize() {
        let text = "Huffman-ft-uyen".to_string();
        let mut encode_result = StandardHuffmanCoding::encode(&text).unwrap();
        let serialize_result = encode_result.serialize();
        assert_eq!(
            serialize_result,
            "1-29-001f01n1-0001y1t1u001a1H01m1em\u{7}1\u{19}\u{17}1t"
        );
    }
}
