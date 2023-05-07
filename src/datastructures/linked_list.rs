pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn push_front(&mut self, data: T) {
        let new_node = Node {
            data: data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_linked_list() {
        let mut L = LinkedList::new();
        L.push_front(1);
        L.push_front(2);
        let test_result = L.size();
        println!("{}",&test_result);
        const ideal_result: usize = 2;
        assert_eq!(test_result,ideal_result);
    }
}
