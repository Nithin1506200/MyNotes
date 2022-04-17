use std::boxed::Box;
struct Node<T> {
    value:T,
    next:Option<Box<Node<T>>>,
    prev:Option<Box<Node<T>>>,
}
struct Deque<T> {
    head: Option<Box<Node<T>>>,
    tail:Option<Box<Node<T>>>,
}

impl<T> Deque<T> where T: Debug {
    fn new() -> Self {
        Self {
            begin:None,
            end: None,
        }
    }

    fn push_fornt(&mut self, value: T) {
        todo!()
    }
    fn push_back(&mut self, value:T) {
        todo!()
    }
    fn debub_dump(&self) {
        let mut iter =self.head;
        while let Some(node)= iter {
            print!("{}",*node)
        }
    }
}
fn main() {
    let mut xs=Deque::<String>::new();
    xs.push_fornt("a".to_string());
    xs.push_back("b".to_string());
    xs.debub_dump();
}