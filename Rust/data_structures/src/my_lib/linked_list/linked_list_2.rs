#[allow(dead_code)]

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(node) => {
                let mut current: &Node<T> = node;

                let mut length: usize = 1;
                while current.next.is_some() {
                    current = current.next.as_ref().unwrap();
                    length = length + 1;
                }
                length
            }
        }
    }

    pub fn push(&mut self, _element: T) {
        let next: Option<Box<Node<T>>> = self.head.take();
        let new_node = Some(Box::new(Node {
            data: _element,
            next,
        }));
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head: Option<Box<Node<T>>> = self.head.take();
        head.map(|x| {
            self.head = x.next;
            x.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut v = SimpleLinkedList::new();
        _item.iter().for_each(|x| v.push(x.clone()));
        v
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut ret_val = SimpleLinkedList::new();
        match &self.head {
            Some(node) => {
                let mut current: &Node<T> = node;

                ret_val.push(current.data.clone());
                while current.next.is_some() {
                    current = current.next.as_ref().unwrap();
                    ret_val.push(current.data.clone());
                }
                ret_val
            }
            _ => ret_val,
        }
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        match self.head {
            Some(node) => {
                let mut v = Vec::new();
                let mut current: Node<T> = *node;

                v.push(current.data);
                while current.next.is_some() {
                    current = *current.next.unwrap();
                    v.push(current.data);
                }

                v.reverse();
                v
            }
            None => vec![],
        }
    }
}
