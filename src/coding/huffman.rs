use super::super::models::huffman_tree::HuffmanTree;
use std::{collections::HashMap, fmt::Error};

pub struct HuffmanEncodingResult {
    codes: Vec<char>,
    encoded_text: String,
}

trait HuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, Error>;
    fn decode() -> Result<HuffmanTree<char>, Error>;
}
pub struct StandardHuffmanCoding {}

impl HuffmanCoding for StandardHuffmanCoding {
    fn encode(text: &String) -> Result<HuffmanEncodingResult, Error> {
        let char_vec: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::get_tree_as_vec(&char_vec);
        let mut coding_map: HashMap<char, String> = HashMap::new();

        let s = char_vec.into_iter().map(|c| {
            if coding_map.contains_key(&c) {
                coding_map.get(&c).unwrap()
            } else {
                let idx = tree.iter().position(|p| *p == c).unwrap();
                let s = "111".to_string();
                "Fdf"
            }
        });
    }
}
