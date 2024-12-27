pub struct Node<T> {
    value: T,
    freq: u8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(
        value: T,
        freq: u8,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    ) -> Self {
        Self {
            value: value,
            freq: freq,
            left: left,
            right: right,
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_freq(&self) -> u8 {
        self.freq
    }

    fn left(&self) -> Option<&Node<T>> {
        self.left.as_deref()
    }

    fn right(&self) -> Option<&Node<T>> {
        self.right.as_deref()
    }

    fn append_left_by_value(&mut self, value: T, freq: u8) {
        let new_node = Some(Box::new(Self::new(value, freq, None, None)));
        self.left = new_node;
    }

    fn append_left_by_node(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node));
    }

    fn append_right_by_value(&mut self, value: T, freq: u8) {
        let new_node = Some(Box::new(Self::new(value, freq, None, None)));
        self.right = new_node;
    }

    fn append_right_by_node(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_char_node_successful() {
        let left_node = Node::new('a', 3, None, None);
        let right_node = Node::new('b', 2, None, None);
        let _root_node = Node::new(
            'c',
            1,
            Some(Box::new(left_node)),
            Some(Box::new(right_node)),
        );
    }

    #[test]
    fn test_init_string_node_successful() {
        let left_node = Node::new("left".to_string(), 3, None, None);
        let right_node = Node::new("right".to_string(), 2, None, None);
        let _root_node = Node::new(
            "root".to_string(),
            1,
            Some(Box::new(left_node)),
            Some(Box::new(right_node)),
        );
    }

    #[test]
    fn test_get_value() {
        let value = "Hello world!";
        let freq: u8 = 2;
        let node = Node::new(value, freq, None, None);

        assert_eq!(*node.get_value(), value);
    }

    #[test]
    fn test_get_fred() {
        let value = "Hello world!";
        let freq: u8 = 2;
        let node = Node::new(value, freq, None, None);

        assert_eq!(node.get_freq(), freq);
    }

    #[test]
    fn test_left_nothing() {
        let node = Node::new(true, 3, None, None);

        assert!(node.left().is_none());
    }

    #[test]
    fn test_left() {
        let left_node = Node::new(Some(true), 3, None, None);
        let node = Node::new(Some(true), 3, Some(Box::new(left_node)), None);

        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), Some(true));
        assert!(node.left().unwrap().left().is_none());
    }

    #[test]
    fn test_right_nothing() {
        let node = Node::new(true, 3, None, None);

        assert!(node.right().is_none());
    }

    #[test]
    fn test_right() {
        let right_node = Node::new(Some(true), 3, None, None);
        let node = Node::new(Some(true), 3, None, Some(Box::new(right_node)));

        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), Some(true));
        assert!(node.right().unwrap().right().is_none());
    }

    #[test]
    fn test_append_left_by_value() {
        let mut node = Node::new(12, 3, None, None);

        let left_value = 34;
        node.append_left_by_value(left_value, 2);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), left_value);

        let new_left_value = 56;
        node.append_left_by_value(new_left_value, 5);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), new_left_value);
    }

    #[test]
    fn test_append_left_by_node() {
        let mut node = Node::new(12, 3, None, None);

        let left_value = 34;
        let left_node = Node::new(left_value, 3, None, None);
        node.append_left_by_node(left_node);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), left_value);

        let new_left_value = 56;
        let new_left_node = Node::new(new_left_value, 3, None, None);
        node.append_left_by_node(new_left_node);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), new_left_value);
    }

    #[test]
    fn test_append_right_by_value() {
        let mut node = Node::new(12, 3, None, None);

        let right_value = 34;
        node.append_right_by_value(right_value, 2);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), right_value);

        let new_right_value = 56;
        node.append_right_by_value(new_right_value, 5);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), new_right_value);
    }

    #[test]
    fn test_append_right_by_node() {
        let mut node = Node::new(12, 3, None, None);

        let right_value = 34;
        let right_node = Node::new(right_value, 3, None, None);
        node.append_right_by_node(right_node);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), right_value);

        let new_right_value = 56;
        let new_right_node = Node::new(new_right_value, 3, None, None);
        node.append_right_by_node(new_right_node);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), new_right_value);
    }
}
