pub struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            left: None,
            right: None}
    }

    fn insert(&mut self,value: T) {
        if value <= self.value {
            match &mut self.left {
                Some(node) => node.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        }
        else {
            match &mut self.right {
                Some(node) => node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    fn search(&self, value: T) -> bool {
        if value == self.value {
            return true
        }
        if value < self.value {
            match &self.left {
                Some(node) => node.search(value),
                None => false
            }
        }
        else {
            match &self.right {
                Some(node) => node.search(value),
                None => false
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_tree() {
        let mut n: Node<i32> = Node::new(2);
        n.insert(1);
        n.insert(2);
        n.insert(3);
        assert_eq!(false,n.search(4));
        assert_eq!(true,n.search(1));
        assert_eq!(true,n.search(2));
        assert_eq!(true,n.search(3));
    }
}
