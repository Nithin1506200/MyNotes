use std::boxed::Box;
 use core::fmt::Debug;

struct Node<T> {
    value:T,
    next:Option<Box<Node<T>>>,
    prev:Option<Box<Node<T>>>,
}
#[derive(Clone)]
struct LinkedList<T> {
    
    head:Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new()-> Self {
        Self {
            head: None,
        }
    }
    fn push_front(&mut self,value:T) {
        let mut head=self.head;
        let new_node= Node{
            value:value,
            next:head,
            prev:None,
        };
        head=Some(Box::new(new_node));


    }
}
fn main() {
    let mut xs=LinkedList::<String>::new();
    println!("Its working");
    
}