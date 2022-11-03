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
    pub fn new(value: T) -> LinkedList<T> {
        let new_head: Node<T> = Node { value, next: None };
        let new: LinkedList<T> = LinkedList {
            head: Some(Box::new(new_head)),
        };
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
 where T:Copy
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
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_test() {
        let new_linked_list: LinkedList<i32> = LinkedList::new(32);
        println!("{:?}", new_linked_list);
    }
    #[test]
    fn push() {
        let mut new_linked_list: LinkedList<i32> = LinkedList::new(90);
        new_linked_list.push(1);
        new_linked_list.push(2);
        new_linked_list.push(3);
        println!(" push {:?}", new_linked_list);
        let popped = new_linked_list.pop();
        println!("popped {:?} ", popped);
        println!("array {:?}", new_linked_list);
    }
}
