struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node { val, next: None }
    }
    pub fn insert_next(&mut self, next: Node<T>) {
        self.next = Some(Box::new(next));
    }
}
