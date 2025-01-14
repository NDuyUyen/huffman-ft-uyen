use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    hash::Hash,
    usize,
};

use crate::errors::huffman_error::HuffmanError;

use super::node::Node;

pub struct HuffmanNode<T> {
    freq: usize,
    value: Option<T>,
}

impl<T> HuffmanNode<T> {
    pub fn new(freq: usize, value: Option<T>) -> Self {
        Self {
            freq: freq,
            value: value,
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

    pub fn get_encoding_map(&self) -> Result<HashMap<T, String>, HuffmanError> {
        let collection: HashMap<T, String> = HashMap::new();
        let based_path = String::new();

        match &self.root {
            Some(root) => Self::collect_paths(Some(root), collection, based_path),
            None => Err(HuffmanError::invalid_huffman_tree()),
        }
    }

    pub fn get_root(&self) -> &Option<Node<HuffmanNode<T>>> {
        &self.root
    }

    fn collect_paths(
        node: Option<&Node<HuffmanNode<T>>>,
        mut collection: HashMap<T, String>,
        path: String,
    ) -> Result<HashMap<T, String>, HuffmanError> {
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
                    let left_result =
                        Self::collect_paths(node.left(), collection, path.clone() + "0").unwrap();
                    Self::collect_paths(node.right(), left_result, path.clone() + "1")
                }
            }
            None => Ok(collection),
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
        let value = "Welcome to my world!!!".as_bytes();
        let tree = HuffmanTree::from(value);

        tree.print_tree_pretty();
    }

    #[test]
    fn test_get_encoding_map() {
        let value = "Welcome to my world!!!".as_bytes();
        let tree = HuffmanTree::from(value);

        let result = tree.get_encoding_map().unwrap();
        let expect: HashMap<u8, String> = HashMap::from([
            (32, "110".to_string()),
            (33, "101".to_string()),
            (111, "100".to_string()),
            (101, "010".to_string()),
            (108, "1111".to_string()),
            (109, "1110".to_string()),
            (87, "0110".to_string()),
            (99, "01111".to_string()),
            (100, "01110".to_string()),
            (114, "0001".to_string()),
            (116, "0000".to_string()),
            (119, "0011".to_string()),
            (121, "0010".to_string()),
        ]);

        assert_eq!(result, expect);
    }
}
