use std::{collections::HashMap, hash::Hash};

use super::node::Node;
struct HuffmanTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> HuffmanTree<T>
where
    T: Eq + Hash + Copy,
{
    pub fn from(values: &[T]) {
        // -> Self
    }

    fn get_freq(values: &[T]) -> HashMap<T, u8> {
        let mut map: HashMap<T, u8> = HashMap::new();
        values
            .into_iter()
            .for_each(|&v| *map.entry(v).or_insert(0) += 1);

        map
    }
}
