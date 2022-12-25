mod linked_list_2;

use std::mem;
#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        let new: LinkedList<T> = LinkedList { head: None };
        new
    }
    pub fn push(&mut self, value: T) {
        let new_tail: Node<T> = Node { value, next: None };
        let mut ptr = &mut self.head;
        loop {
            match ptr {
                None => {
                    *ptr = Some(Box::new(new_tail));
                    break;
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }
    }
    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let mut ptr = &mut self.head;
        loop {
            match ptr {
                None => {
                    return None;
                }
                Some(node) if node.next.is_none() => {
                    let val = node.value;
                    *ptr = None;
                    return Some(val);
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }
    }
    /** drop is not needed in rust as it auto clears */
    pub fn drop(&mut self) {
        let mut ptr = mem::replace(&mut self.head, None);
        while let Some(mut node) = ptr {
            ptr = mem::replace(&mut node.next, None);
        }
    }
    pub fn get_tail(&mut self) -> Option<&mut Node<T>> {
        let mut ptr = &mut self.head;
        loop {
            match ptr.as_mut() {
                None => {
                    return None;
                }
                Some(node) if (node.next.is_none()) => return Some(node),
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }
    }
}
#[macro_export]
macro_rules! linkedlist {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = LinkedList::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_test() {
        let new_linked_list: LinkedList<i32> = LinkedList::new();
        println!("{:?}", new_linked_list);
    }
    #[test]
    fn push() {
        let mut new_linked_list: LinkedList<i32> = LinkedList::new();
        new_linked_list.push(1);
        new_linked_list.push(2);
        new_linked_list.push(3);
        println!(" push {:?}", new_linked_list);
        let popped = new_linked_list.pop();
        println!("popped {:?} ", popped);
        println!("array {:?}", new_linked_list);
    }
    #[test]
    fn get_tail() {
        let mut new_linked_list: LinkedList<i32> = LinkedList::new();
        new_linked_list.push(1);
        new_linked_list.push(2);
        new_linked_list.push(3);
        println!("{:?}", new_linked_list.get_tail());
    }
    #[test]
    fn macro_test() {
        let ll = linkedlist!(5, 6, 7, 8);
        println!("{:?}", ll);
    }
}
