use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    hash::Hash,
    slice::Iter,
    usize,
};

use crate::errors::huffman_error::HuffmanError;

use super::node::Node;
#[derive(Clone)]
pub struct HuffmanNode<T> {
    freq: usize,
    value: Option<T>,
}

impl<T> HuffmanNode<T>
where
    T: ToString,
{
    pub fn new(freq: usize, value: Option<T>) -> Self {
        Self {
            freq: freq,
            value: value,
        }
    }

    pub fn get_value_as_string(&self) -> String {
        match &self.value {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    }
}

impl<T> ToString for HuffmanNode<T>
where
    T: Display,
{
    fn to_string(&self) -> String {
        let result = match &self.value {
            Some(c) => format!("C: {} - F:{}", c, self.freq),
            None => format!("None - F:{}", self.freq),
        };
        result
    }
}

#[derive(Clone)]
pub struct HuffmanTree<T> {
    root: Option<Node<HuffmanNode<T>>>,
}

impl<T> HuffmanTree<T>
where
    T: Eq + Hash + Copy + Ord + ToString,
    HuffmanNode<T>: ToString,
{
    pub fn from(values: &[T]) -> Self {
        let leaves = Self::build_node_leaves_vec(values);
        let tree = Self::build_tree(leaves);

        Self { root: tree }
    }

    pub fn get_root(&self) -> &Option<Node<HuffmanNode<T>>> {
        &self.root
    }

    pub fn get_encoding_map(&self) -> Result<HashMap<T, Vec<bool>>, HuffmanError> {
        let collection: HashMap<T, Vec<bool>> = HashMap::new();
        let based_path: Vec<bool> = Vec::new();

        match &self.root {
            Some(root) => Self::collect_paths(Some(root), collection, based_path),
            None => Err(HuffmanError::invalid_huffman_tree()),
        }
    }

    pub fn decode_by_path(&self, iter: &mut Iter<'_, bool>) -> Result<T, HuffmanError> {
        match &self.root {
            Some(root) => Self::get_value_by_path(Some(root), iter),
            None => Err(HuffmanError::invalid_huffman_tree()),
        }
    }

    fn collect_paths(
        node: Option<&Node<HuffmanNode<T>>>,
        mut collection: HashMap<T, Vec<bool>>,
        path: Vec<bool>,
    ) -> Result<HashMap<T, Vec<bool>>, HuffmanError> {
        match node {
            Some(node) => {
                if node.is_leaf() {
                    match node.get_value().value {
                        Some(v) => {
                            collection.insert(v, path);
                            Ok(collection)
                        }
                        None => Err(HuffmanError::invalid_huffman_tree()),
                    }
                } else {
                    let mut left_based_path = path.clone();
                    left_based_path.push(false);
                    let left_result =
                        Self::collect_paths(node.left(), collection, left_based_path).unwrap();
                    let mut right_based_path = path.clone();
                    right_based_path.push(true);
                    Self::collect_paths(node.right(), left_result, right_based_path)
                }
            }
            None => Ok(collection),
        }
    }

    fn get_value_by_path(
        node: Option<&Node<HuffmanNode<T>>>,
        iter: &mut Iter<'_, bool>,
    ) -> Result<T, HuffmanError> {
        match node {
            Some(node) => {
                if node.is_leaf() {
                    match node.get_value().value {
                        Some(v) => Ok(v),
                        None => Err(HuffmanError::invalid_huffman_tree()),
                    }
                } else {
                    match iter.next() {
                        Some(direction) => {
                            if *direction {
                                // right
                                Self::get_value_by_path(node.right(), iter)
                            } else {
                                // left
                                Self::get_value_by_path(node.left(), iter)
                            }
                        }
                        None => Err(HuffmanError::decoding_error()),
                    }
                }
            }
            None => Err(HuffmanError::invalid_huffman_tree()),
        }
    }

    fn combine(
        left: Option<Node<HuffmanNode<T>>>,
        right: Option<Node<HuffmanNode<T>>>,
    ) -> Node<HuffmanNode<T>> {
        let mut new_freq: usize = 0;
        let left_node = match left {
            Some(node) => {
                new_freq += node.get_value().freq;
                Some(Box::new(node))
            }
            None => None,
        };
        let right_node = match right {
            Some(node) => {
                new_freq += node.get_value().freq;
                Some(Box::new(node))
            }
            None => None,
        };

        Node::new(HuffmanNode::new(new_freq, None), left_node, right_node)
    }

    fn build_tree(mut nodes: Vec<Node<HuffmanNode<T>>>) -> Option<Node<HuffmanNode<T>>>
    where
        T: ToString,
    {
        let mut root: Option<Node<HuffmanNode<T>>> = None;

        while nodes.len() > 0 {
            if nodes.len() == 1 {
                let min = nodes.pop().unwrap();
                root = Some(Self::combine(Some(min), None));
                break;
            } else {
                let min_1 = nodes.pop().unwrap();
                let min_2 = nodes.pop().unwrap();
                let new_node = Self::combine(Some(min_1), Some(min_2));

                if nodes.len() == 0 {
                    root = Some(new_node);
                    break;
                } else {
                    nodes = Self::add_node(nodes, new_node);
                }
            }
        }

        root
    }

    fn get_freq_using_hashmap(values: &[T]) -> HashMap<T, usize> {
        let mut map: HashMap<T, usize> = HashMap::new();
        values
            .into_iter()
            .for_each(|&v| *map.entry(v).or_insert(0) += 1);

        map
    }

    fn get_freq_using_btreemap(values: &[T]) -> BTreeMap<T, usize> {
        let mut map: BTreeMap<T, usize> = BTreeMap::new();
        values
            .into_iter()
            .for_each(|&v| *map.entry(v).or_insert(0) += 1);

        map
    }

    fn build_node_leaves_vec(values: &[T]) -> Vec<Node<HuffmanNode<T>>> {
        let map = Self::get_freq_using_btreemap(values);
        let mut hash_vec: Vec<(T, usize)> = map.into_iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(&a.1));
        hash_vec
            .into_iter()
            .map(|a| Node::new(HuffmanNode::new(a.1, Some(a.0)), None, None))
            .collect()
    }

    fn add_node(
        mut nodes: Vec<Node<HuffmanNode<T>>>,
        new_node: Node<HuffmanNode<T>>,
    ) -> Vec<Node<HuffmanNode<T>>> {
        nodes.push(new_node);
        nodes.sort_by(|a, b| b.get_value().freq.cmp(&a.get_value().freq));
        nodes
    }

    fn print_tree_pretty(&self) {
        match self.get_root() {
            Some(root) => root.represent_tree(),
            None => {}
        }
    }
}

impl HuffmanTree<char> {
    const CHAR_PARENT_NODE: char = '0';
    const CHAR_LEAF_NODE: char = '1';

    pub fn serialize(&self) -> String {
        match &self.root {
            Some(root) => Self::serialize_internal(root),
            None => String::new(),
        }
    }

    pub fn deserialize(text: String) -> Result<HuffmanTree<char>, HuffmanError> {
        let chars_vec: Vec<char> = text.chars().collect();
        let mut iter: std::slice::Iter<'_, char> = chars_vec.iter();

        if iter.len() > 0 {
            let root = HuffmanTree::deserialize_internal(&mut iter);

            if iter.len() > 0 {
                Err(HuffmanError::cannot_deserialize_tree())
            } else {
                match root {
                    Ok(root) => Ok(Self { root: root }),
                    Err(e) => Err(e),
                }
            }
        } else {
            Ok(Self { root: None })
        }
    }

    fn serialize_internal(node: &Node<HuffmanNode<char>>) -> String {
        if node.is_leaf() {
            return Self::CHAR_LEAF_NODE.to_string() + &node.get_value().get_value_as_string();
        } else {
            let left_str = match node.left() {
                Some(left_node) => Self::serialize_internal(left_node),
                None => String::new(),
            };
            let right_str = match node.right() {
                Some(right_node) => Self::serialize_internal(right_node),
                None => String::new(),
            };

            return Self::CHAR_PARENT_NODE.to_string() + &left_str + &right_str;
        }
    }

    fn deserialize_internal(
        iter: &mut Iter<'_, char>,
    ) -> Result<Option<Node<HuffmanNode<char>>>, HuffmanError> {
        match iter.next() {
            Some(n) => {
                if *n == Self::CHAR_PARENT_NODE {
                    let mut has_child = false;
                    let left = match HuffmanTree::deserialize_internal(iter)? {
                        Some(l) => {
                            has_child = true;
                            Some(Box::new(l))
                        }
                        None => None,
                    };
                    let right = match HuffmanTree::deserialize_internal(iter)? {
                        Some(r) => {
                            has_child = true;
                            Some(Box::new(r))
                        }
                        None => None,
                    };

                    if has_child {
                        Ok(Some(Node::new(HuffmanNode::new(0, None), left, right)))
                    } else {
                        Err(HuffmanError::cannot_deserialize_tree())
                    }
                } else if *n == Self::CHAR_LEAF_NODE {
                    match iter.next() {
                        Some(c) => Ok(Some(Node::new(HuffmanNode::new(0, Some(*c)), None, None))),
                        None => Err(HuffmanError::cannot_deserialize_tree()),
                    }
                } else {
                    Err(HuffmanError::cannot_deserialize_tree())
                }
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_get_freq_using_hashmap() {
        let value = "Welcome to my world!!!".as_bytes();
        // [87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 109, 121, 32, 119, 111, 114, 108, 100, 33, 33, 33]
        let result = HuffmanTree::get_freq_using_hashmap(value);
        let expect: HashMap<u8, usize> = HashMap::from([
            (87, 1),
            (101, 2),
            (108, 2),
            (99, 1),
            (111, 3),
            (109, 2),
            (32, 3),
            (116, 1),
            (121, 1),
            (119, 1),
            (114, 1),
            (33, 3),
            (100, 1),
        ]);
        assert_eq!(result, expect);

        let result = HuffmanTree::get_freq_using_hashmap("".as_bytes());
        let expect: HashMap<u8, usize> = HashMap::new();
        assert_eq!(result, expect);

        let result = HuffmanTree::get_freq_using_hashmap(&[2, 5, 3, 3, 2, 4, 5]);
        let expect: HashMap<u8, usize> = HashMap::from([(2, 2), (3, 2), (4, 1), (5, 2)]);
        assert_eq!(result, expect);
    }

    #[test]
    fn test_get_freq_using_btreemap() {
        let value = "Welcome to my world!!!".as_bytes();
        // [87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 109, 121, 32, 119, 111, 114, 108, 100, 33, 33, 33]
        let result = HuffmanTree::get_freq_using_btreemap(value);
        let expect: BTreeMap<u8, usize> = BTreeMap::from([
            (87, 1),
            (101, 2),
            (108, 2),
            (99, 1),
            (111, 3),
            (109, 2),
            (32, 3),
            (116, 1),
            (121, 1),
            (119, 1),
            (114, 1),
            (33, 3),
            (100, 1),
        ]);
        assert_eq!(result, expect);

        let result = HuffmanTree::get_freq_using_btreemap("".as_bytes());
        let expect: BTreeMap<u8, usize> = BTreeMap::new();
        assert_eq!(result, expect);

        let result = HuffmanTree::get_freq_using_btreemap(&[2, 5, 3, 3, 2, 4, 5]);
        let expect: BTreeMap<u8, usize> = BTreeMap::from([(2, 2), (3, 2), (4, 1), (5, 2)]);
        assert_eq!(result, expect);
    }

    #[test]
    fn test_build_node_leaves_vec() {
        let value = "Welcome to my world!!!".as_bytes();
        let result = HuffmanTree::build_node_leaves_vec(value);
        let chars_result: Vec<u8> = result
            .iter()
            .map(|node| node.get_value().value.unwrap())
            .collect();
        let chars_expect = &[32, 33, 111, 101, 108, 109, 87, 99, 100, 114, 116, 119, 121];
        assert_eq!(chars_result, chars_expect);

        let freqs_result: Vec<usize> = result.iter().map(|node| node.get_value().freq).collect();
        let freqs_expect: &[usize; 13] = &[3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(freqs_result, freqs_expect);
    }

    #[test]
    fn test_combine() {
        let left = Node::new(HuffmanNode::new(3, Some(32)), None, None);
        let right = Node::new(HuffmanNode::new(1, Some(12)), None, None);
        let root = HuffmanTree::combine(Some(left), Some(right));

        assert_eq!(root.get_value().freq, 4);
        assert!(root.get_value().value.is_none());
        assert_eq!(root.left().unwrap().get_value().freq, 3);
        assert_eq!(root.right().unwrap().get_value().freq, 1);

        let left = Node::new(HuffmanNode::new(3, Some(32)), None, None);
        let root = HuffmanTree::combine(Some(left), None);

        assert_eq!(root.get_value().freq, 3);
        assert_eq!(root.left().unwrap().get_value().freq, 3);
        assert_eq!(root.left().unwrap().get_value().value, Some(32));
        assert!(root.right().is_none());

        let right = Node::new(HuffmanNode::new(3, Some(32)), None, None);
        let root = HuffmanTree::combine(None, Some(right));

        assert_eq!(root.get_value().freq, 3);
        assert_eq!(root.right().unwrap().get_value().freq, 3);
        assert_eq!(root.right().unwrap().get_value().value, Some(32));
        assert!(root.left().is_none());

        let root: Node<HuffmanNode<u8>> = HuffmanTree::combine(None, None);

        assert_eq!(root.get_value().freq, 0);
        assert!(root.left().is_none());
        assert!(root.right().is_none());
    }

    #[test]
    fn test_combine_complex() {
        let left_left = Node::new(HuffmanNode::new(1, Some(1)), None, None);
        let left_right = Node::new(HuffmanNode::new(2, Some(3)), None, None);
        let left = HuffmanTree::combine(Some(left_left), Some(left_right));
        let right = Node::new(HuffmanNode::new(4, Some(5)), None, None);
        let root = HuffmanTree::combine(Some(left), Some(right));

        assert_eq!(root.get_value().freq, 7);
        assert!(root.get_value().value.is_none());
        assert_eq!(root.left().unwrap().get_value().freq, 3);
        assert!(root.left().unwrap().get_value().value.is_none());
        assert_eq!(root.right().unwrap().get_value().freq, 4);
        assert_eq!(root.right().unwrap().get_value().value, Some(5));
        assert_eq!(root.left().unwrap().left().unwrap().get_value().freq, 1);
        assert_eq!(
            root.left().unwrap().left().unwrap().get_value().value,
            Some(1)
        );
        assert_eq!(root.left().unwrap().right().unwrap().get_value().freq, 2);
        assert_eq!(
            root.left().unwrap().right().unwrap().get_value().value,
            Some(3)
        );
    }

    #[test]
    fn test_build_tree() {
        let value = "Welcome to my world!!!".as_bytes();
        let node_leaves = HuffmanTree::build_node_leaves_vec(value);
        let tree = HuffmanTree::build_tree(node_leaves);

        assert!(tree.is_some());
        assert_eq!(tree.unwrap().get_value().freq, 22);
    }

    #[test]
    fn test_from() {
        let text = "Welcome to my world!!!".to_string();
        let text_as_chars: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::from(&text_as_chars);

        tree.print_tree_pretty();
    }

    #[test]
    fn test_get_encoding_map() {
        let value = "Welcome to my world!!!".as_bytes();
        let tree = HuffmanTree::from(value);

        let result = tree.get_encoding_map().unwrap();
        let l = false;
        let r = true;
        let expect: HashMap<u8, Vec<bool>> = HashMap::from([
            (32, vec![r, r, l]),
            (33, vec![r, l, r]),
            (111, vec![r, l, l]),
            (101, vec![l, r, l]),
            (108, vec![r, r, r, r]),
            (109, vec![r, r, r, l]),
            (87, vec![l, r, r, l]),
            (99, vec![l, r, r, r, r]),
            (100, vec![l, r, r, r, l]),
            (114, vec![l, l, l, r]),
            (116, vec![l, l, l, l]),
            (119, vec![l, l, r, r]),
            (121, vec![l, l, r, l]),
        ]);

        assert_eq!(result, expect);
    }

    #[test]
    fn test_decode_by_path_successful() {
        let text = "Welcome to my world!!!".to_string();
        let text_as_chars: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::from(&text_as_chars);
        let l = false;
        let r = true;
        let encoded_vec = vec![
            l, r, r, l, l, r, l, r, r, r, r, l, r, r, r, r, r, l, l, r, r, r, l, l, r, l, r, r, l,
            l, l, l, l, r, l, l, r, r, l, r, r, r, l, l, l, r, l, r, r, l, l, l, r, r, r, l, l, l,
            l, l, r, r, r, r, r, l, r, r, r, l, r, l, r, r, l, r, r, l, r,
        ];
        let mut decoded_text = String::new();
        let mut iter = encoded_vec.iter();
        for _ in 1..23 {
            decoded_text.push(tree.decode_by_path(&mut iter).unwrap());
        }

        assert_eq!(decoded_text, text);
    }

    #[test]
    fn test_decode_by_path_failed() {
        let text = "Welcome to my world!!!".to_string();
        let text_as_chars: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::from(&text_as_chars);
        let l = false;
        let r = true;
        let encoded_vec = vec![l, r];
        let mut iter = encoded_vec.iter();

        assert!(tree.decode_by_path(&mut iter).is_err());
    }

    #[test]
    fn test_serialize() {
        let text = "Welcome to my world!!!".to_string();
        let text_as_chars: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::from(&text_as_chars);
        let result = tree.serialize();
        let expect = "00001t1r01y1w01e01W01d1c001o1!01 01m1l".to_string();

        assert_eq!(result, expect);
    }

    #[test]
    fn test_deserialize_successful() {
        let input = "00001t1r01y1w01e01W01d1c001o1!01 01m1l".to_string();
        let tree = HuffmanTree::deserialize(input.clone());
        let serialized_text = tree.unwrap().serialize();

        assert_eq!(serialized_text, input);

        let input = "00001t1r01y1w01e01W01d1c001o1!01 01m".to_string();
        let tree = HuffmanTree::deserialize(input.clone());
        let serialized_text = tree.unwrap().serialize();

        assert_eq!(serialized_text, input);

        let input = "1c".to_string();
        let tree = HuffmanTree::deserialize(input.clone());
        let serialized_text = tree.unwrap().serialize();

        assert_eq!(serialized_text, input);

        let input = String::new();
        let tree = HuffmanTree::deserialize(input.clone());
        let serialized_text = tree.unwrap().serialize();

        assert_eq!(serialized_text, input);

        let text = "Welcome to my world - 12312121 00 233 ~@#$%%& #fbdfd af !!!".to_string();
        let text_as_chars: Vec<char> = text.chars().collect();
        let tree = HuffmanTree::from(&text_as_chars);
        let serialized_text_1 = tree.serialize();
        let deserialized_tree = HuffmanTree::deserialize(serialized_text_1.clone());
        let serialized_text_2 = deserialized_tree.unwrap().serialize();

        assert_eq!(serialized_text_1, serialized_text_2);
    }

    #[test]
    fn test_deserialize_failed() {
        let input = "00001t1r01y1w01e01W01d1c001o1!01 01m1l03".to_string();
        let tree = HuffmanTree::deserialize(input);
        assert!(tree.is_err());

        let input = "00001t1r01y1w01e01W01d1c001o1!01 0".to_string();
        let tree = HuffmanTree::deserialize(input);
        assert!(tree.is_err());

        let input = "00001t1r01y1w01e01W01d1c001o1!01 01m1".to_string();
        let tree = HuffmanTree::deserialize(input);
        assert!(tree.is_err());

        let input = "00001t1r01y1w01e01W01d1c001o1!1.1,".to_string();
        let tree = HuffmanTree::deserialize(input);
        assert!(tree.is_err());

        let input = "00001t1r01y1w01e01W01d1c00".to_string();
        let tree = HuffmanTree::deserialize(input);
        assert!(tree.is_err());
    }
}
