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

    fn append_left(&mut self, value: T, freq: u8) {
        let new_node = Some(Box::new(Self::new(value, freq, None, None)));
        self.left = new_node;
    }

    fn append_right(&mut self, value: T, freq: u8) {
        let new_node = Some(Box::new(Self::new(value, freq, None, None)));
        self.right = new_node;
    }
}
