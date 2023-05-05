use std::mem;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push_front(&mut self,data: T) {
        let new_node: T = Node {
            data,
            next: self.head.take()
        };

        self.head = Some(Box::new(new_node));
        self.size += 1;
    }
}

