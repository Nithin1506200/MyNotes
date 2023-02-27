struct Node<'a, T> {
    val: &'a T,
    l: Option<Box<Node<'a, T>>>,
    r: Option<Box<Node<'a, T>>>,
}
