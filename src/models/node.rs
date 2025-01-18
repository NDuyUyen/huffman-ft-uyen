#[derive(Clone)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: ToString,
{
    pub fn new(value: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Self {
        Self {
            value: value,
            left: left,
            right: right,
        }
    }

    fn get_value_str(&self) -> String {
        self.value.to_string()
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&Node<T>> {
        self.left.as_deref()
    }

    pub fn right(&self) -> Option<&Node<T>> {
        self.right.as_deref()
    }

    pub fn append_left_by_value(&mut self, value: T) {
        let new_node = Some(Box::new(Self::new(value, None, None)));
        self.left = new_node;
    }

    pub fn append_left_by_node(&mut self, node: Node<T>) {
        self.left = Some(Box::new(node));
    }

    pub fn append_right_by_value(&mut self, value: T) {
        let new_node = Some(Box::new(Self::new(value, None, None)));
        self.right = new_node;
    }

    pub fn append_right_by_node(&mut self, node: Node<T>) {
        self.right = Some(Box::new(node));
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn has_child(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }

    pub fn represent_tree(&self) {
        println!("{}", self.get_value_str());
        self.represent_children("".to_string());
        println!("");
    }
    fn represent_children(&self, prefix: String) {
        let left = self.left();
        let right = self.right();

        if !self.has_child() {
            return;
        } else {
            print!("{}", prefix);
            if right.is_some() {
                if left.is_some() {
                    print!("├── ");
                } else {
                    print!("└── ");
                }
            }
        }

        match right {
            Some(right) => {
                let new_prefix = prefix.clone()
                    + (if left.is_some() && right.has_child() {
                        "│   "
                    } else {
                        "    "
                    });
                println!("{}", right.get_value_str());
                right.represent_children(new_prefix);
            }
            None => {}
        }

        match left {
            Some(left) => {
                if right.is_some() {
                    print!("{}", prefix.clone());
                }
                print!("└── ");
                println!("{}", left.get_value_str());
                left.represent_children(prefix + "    ");
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_char_node_successful() {
        let left_node = Node::new('a', None, None);
        let right_node = Node::new('b', None, None);
        let _root_node = Node::new('c', Some(Box::new(left_node)), Some(Box::new(right_node)));
    }

    #[test]
    fn test_init_string_node_successful() {
        let left_node = Node::new("left".to_string(), None, None);
        let right_node = Node::new("right".to_string(), None, None);
        let _root_node = Node::new(
            "root".to_string(),
            Some(Box::new(left_node)),
            Some(Box::new(right_node)),
        );
    }

    #[test]
    fn test_get_value() {
        let value = "Hello world!";
        let node = Node::new(value, None, None);

        assert_eq!(*node.get_value(), value);
    }

    #[test]
    fn test_left_nothing() {
        let node = Node::new(true, None, None);

        assert!(node.left().is_none());
    }

    #[test]
    fn test_left() {
        let left_node = Node::new(true, None, None);
        let node = Node::new(true, Some(Box::new(left_node)), None);

        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), true);
        assert!(node.left().unwrap().left().is_none());
    }

    #[test]
    fn test_right_nothing() {
        let node = Node::new(true, None, None);

        assert!(node.right().is_none());
    }

    #[test]
    fn test_right() {
        let right_node = Node::new(true, None, None);
        let node = Node::new(true, None, Some(Box::new(right_node)));

        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), true);
        assert!(node.right().unwrap().right().is_none());
    }

    #[test]
    fn test_append_left_by_value() {
        let mut node = Node::new(12, None, None);

        let left_value = 34;
        node.append_left_by_value(left_value);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), left_value);

        let new_left_value = 56;
        node.append_left_by_value(new_left_value);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), new_left_value);
    }

    #[test]
    fn test_append_left_by_node() {
        let mut node = Node::new(12, None, None);

        let left_value = 34;
        let left_node = Node::new(left_value, None, None);
        node.append_left_by_node(left_node);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), left_value);

        let new_left_value = 56;
        let new_left_node = Node::new(new_left_value, None, None);
        node.append_left_by_node(new_left_node);
        assert!(node.left().is_some());
        assert_eq!(*node.left().unwrap().get_value(), new_left_value);
    }

    #[test]
    fn test_append_right_by_value() {
        let mut node = Node::new(12, None, None);

        let right_value = 34;
        node.append_right_by_value(right_value);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), right_value);

        let new_right_value = 56;
        node.append_right_by_value(new_right_value);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), new_right_value);
    }

    #[test]
    fn test_append_right_by_node() {
        let mut node = Node::new(12, None, None);

        let right_value = 34;
        let right_node = Node::new(right_value, None, None);
        node.append_right_by_node(right_node);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), right_value);

        let new_right_value = 56;
        let new_right_node = Node::new(new_right_value, None, None);
        node.append_right_by_node(new_right_node);
        assert!(node.right().is_some());
        assert_eq!(*node.right().unwrap().get_value(), new_right_value);
    }
}
