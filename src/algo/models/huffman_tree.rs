use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    usize,
};

use super::node::Node;
struct HuffmanTree<T> {
    root: Option<Node<T>>,
}

impl<T> HuffmanTree<T>
where
    T: Eq + Hash + Copy + Ord + ToString,
{
    pub fn from(values: &[T]) -> Self {
        let map = Self::get_freq_using_btreemap(values);
        let sorted_vec = Self::sort_map_by_freq(map);
        let tree = Self::build_tree(sorted_vec);

        Self { root: tree }
    }

    pub fn get_root(&self) -> &Option<Node<T>> {
        &self.root
    }

    fn build_tree(mut vec: Vec<T>) -> Option<Node<T>> {
        let largest = vec.pop();

        match largest {
            Some(value) => {
                let right = Node::new(Some(value), None, None);
                let left = Self::build_tree(vec);

                match left {
                    Some(left) => {
                        Some(Node::new(None, Some(Box::new(left)), Some(Box::new(right))))
                    }
                    None => Some(right),
                }
            }
            None => None,
        }
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

    fn sort_map_by_freq(map: BTreeMap<T, usize>) -> Vec<T> {
        let mut hash_vec: Vec<(T, usize)> = map.into_iter().collect();
        hash_vec.sort_by(|a, b| a.1.cmp(&b.1));
        hash_vec.into_iter().map(|a| a.0).collect()
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
    fn test_sort_map_by_freq() {
        let map: BTreeMap<u8, usize> = BTreeMap::from([
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

        let result = HuffmanTree::sort_map_by_freq(map);
        let expect: Vec<u8> = vec![87, 99, 100, 114, 116, 119, 121, 101, 108, 109, 32, 33, 111];
        assert_eq!(result, expect);
    }

    #[test]
    fn test_build_tree() {
        let vec = vec!['a', 'b', 'c', 'd', 'e'];
        let tree = HuffmanTree::build_tree(vec);
        let tree_ref = tree.as_ref().unwrap();

        assert!(tree.is_some());
        assert_eq!(*tree_ref.right().unwrap().get_value(), Some('e'));
        assert_eq!(
            *tree_ref.left().unwrap().right().unwrap().get_value(),
            Some('d')
        );
        assert_eq!(
            *tree_ref
                .left()
                .unwrap()
                .left()
                .unwrap()
                .right()
                .unwrap()
                .get_value(),
            Some('c')
        );
        assert_eq!(
            *tree_ref
                .left()
                .unwrap()
                .left()
                .unwrap()
                .left()
                .unwrap()
                .right()
                .unwrap()
                .get_value(),
            Some('b')
        );
        assert_eq!(
            *tree_ref
                .left()
                .unwrap()
                .left()
                .unwrap()
                .left()
                .unwrap()
                .left()
                .unwrap()
                .get_value(),
            Some('a')
        );
        assert!(tree_ref
            .left()
            .unwrap()
            .left()
            .unwrap()
            .left()
            .unwrap()
            .left()
            .unwrap()
            .left()
            .is_none());

        let vec: Vec<u8> = vec![123];
        let tree = HuffmanTree::build_tree(vec);
        let tree_ref = tree.as_ref().unwrap();

        assert!(tree.is_some());
        assert_eq!(*tree_ref.get_value(), Some(123));
        assert!(tree_ref.left().is_none());
        assert!(tree_ref.right().is_none());

        let vec: Vec<u8> = vec![];
        let tree = HuffmanTree::build_tree(vec);

        assert!(tree.is_none());
    }

    #[test]
    fn test_from() {
        let vec = "eeebeebcadcd".as_bytes();
        let tree = HuffmanTree::from(vec);
        let root = tree.get_root();
        let tree_ref = root.as_ref().unwrap();

        assert!(root.is_some());
        assert_eq!(*tree_ref.right().unwrap().get_value(), Some(101));
        assert_eq!(
            *tree_ref.left().unwrap().right().unwrap().get_value(),
            Some(100)
        );
        assert_eq!(
            *tree_ref
                .left()
                .unwrap()
                .left()
                .unwrap()
                .right()
                .unwrap()
                .get_value(),
            Some(99)
        );
        assert_eq!(
            *tree_ref
                .left()
                .unwrap()
                .left()
                .unwrap()
                .left()
                .unwrap()
                .right()
                .unwrap()
                .get_value(),
            Some(98)
        );
        assert_eq!(
            *tree_ref
                .left()
                .unwrap()
                .left()
                .unwrap()
                .left()
                .unwrap()
                .left()
                .unwrap()
                .get_value(),
            Some(97)
        );
        assert!(tree_ref
            .left()
            .unwrap()
            .left()
            .unwrap()
            .left()
            .unwrap()
            .left()
            .unwrap()
            .left()
            .is_none());

        let vec = "U".as_bytes();
        let tree = HuffmanTree::from(vec);
        let root = tree.get_root();
        let tree_ref = root.as_ref().unwrap();

        assert!(root.is_some());
        assert_eq!(*tree_ref.get_value(), Some(85));

        let vec = "".as_bytes();
        let tree = HuffmanTree::from(vec);
        let root = tree.get_root();

        assert!(root.is_none());
    }

    #[test]
    fn test_print_tree() {
        let vec: Vec<char> = "Welcome to my world".chars().collect();
        let tree = HuffmanTree::from(&vec);

        tree.print_tree_pretty();

        let vec = "eeebeebcadcd".as_bytes();
        let tree = HuffmanTree::from(&vec);

        tree.print_tree_pretty();
    }
}
