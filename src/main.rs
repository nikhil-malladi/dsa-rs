struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.size -= 1;
            node.data
        })
    }

    fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        self.size
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    while let Some(value) = list.pop_front() {
        println!("{}", value);
    }
}
