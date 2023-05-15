type Link = Option<Box<Node>>;
#[derive(Debug)]
struct LinkedList {
    head: Link,
}
#[derive(Debug)]
struct Node {
    element: u32,
    next: Link,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }
    fn push(&mut self, element: u32) {
        // let old_head = std::mem::replace(&mut self.head, None);
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);

        // it doesn't work
        //     match self.head {
        //         Some(head) => {
        //             let new_head = Node {
        //                 element,
        //                 next: Some(head),
        //             };
        //             self.head = Some(Box::new(new_head));
        //         }
        //         None => {
        //             self.head = Some(Box::new(Node {
        //                 element,
        //                 next: None,
        //             }))
        //         }
        //     }
    }

    fn pop(&mut self) -> Option<u32> {
        let old_head = self.head.take();
        match old_head {
            Some(head) => {
                self.head = head.next;
                Some(head.element)
            }
            None => None,
        }
        // another method
        // self.head.take().map(|n| {
        //     self.head = n.next;
        //     n.element
        // })
    }
    fn peek(&self) -> Option<&u32> {
        // add & just to look at it not take ownership of it
        //or impliment copy trait
        match &self.head {
            Some(head) => Some(&head.element),
            None => None,
        }
        //another method
        // self.head.as_ref().map(|n| &n.element)
    }
    fn push_back(&mut self, element: u32) {
        let mut current_node = &mut self.head;

        while let Some(node) = current_node {
            current_node = &mut node.next;
        }

        *current_node = Some(Box::new(Node {
            element,
            next: None,
        }));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = LinkedList { head: None };
        println!(" the head is {:?}", list.head);
        let head = Node {
            element: 3,
            next: None,
        };
        list.head = Some(Box::new(head));
        println!("the new head is {:?}", list.head)
    }
    #[test]
    fn impls() {
        let list = LinkedList::new();
        println!("the list is {:?}", list)
    }
}
