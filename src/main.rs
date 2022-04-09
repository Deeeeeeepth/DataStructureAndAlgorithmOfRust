use std::thread::panicking;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
    len: u32,
}
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None, len: 0 }
    }

    pub fn insert_at_head(&mut self, value: T) {
        let mut node = Node::new(value);
        node.next = self.head.take();

        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn insert_at_tail(&mut self, value: T) {
        if self.len == 0 {
            self.insert_at_head(value);
            return;
        }

        let mut cur = self.head.as_mut().unwrap();
        for _ in 0..self.len - 1 {
            cur = cur.next.as_mut().unwrap();
        }

        cur.next = Some(Box::new(Node::new(value)));
        self.len += 1;
    }

    pub fn insert_at_ith(&mut self, value: T, index: u32) {
        if index > self.len {
            panic!("out of limit! length is {}", self.len);
        }

        if index == 0 {
            self.insert_at_head(value);
            return;
        }

        let mut node = Node::new(value);
        let mut cur = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            cur = cur.next.as_mut().unwrap();
        }

        node.next = cur.next.take();
        cur.next = Some(Box::new(node));

        self.len += 1;
    }

    pub fn delete_head(&mut self) {
        if self.len == 0 {
            return;
        }

        self.head = self.head.as_mut().unwrap().next.take();
        self.len -= 1;
    }

    pub fn delete_tail(&mut self) {
        if self.len == 0 {
            return;
        }

        if self.len == 1 {
            self.head.take();
            return;
        }

        let mut cur = self.head.as_mut().unwrap();

        for _ in 0..self.len - 2 {
            cur = cur.next.as_mut().unwrap();
        }

        cur.next.take();
        self.len -= 1;
    }

    pub fn delete_ith(&mut self, index: u32) {
        if index > self.len - 1 {
            panic!("out of range. legth is {}", self.len);
        }

        let mut cur = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            cur = cur.next.as_mut().unwrap();
        }

        cur.next = cur.next.take();
        self.len -= 1;
    }

    pub fn get(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn get_ith(&self, index: u32) -> Option<&T> {
        if index > self.len - 1 {
            panic!("out of range, length is {}", self.len);
        }

        let mut cur = self.head.as_ref().unwrap();
        for _ in 0..index {
            cur = cur.next.as_ref().unwrap();
        }

        Some(&cur.value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_at_head_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_head(1);
        assert!(list.head.as_ref().unwrap().value == 1);
        assert!(list.len == 1);

        list.insert_at_head(2);
        assert!(list.head.as_ref().unwrap().value == 2);
        assert!(list.len == 2);
    }

    #[test]
    fn insert_at_tail_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_tail(1);
        assert!(list.head.as_ref().unwrap().value == 1);
        assert!(list.len == 1);

        list.insert_at_tail(2);
        assert!(list.head.as_ref().unwrap().value == 1);
        assert!(list.len == 2);
    }

    #[test]
    fn insert_at_ith_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_ith(1, 0);
        assert!(list.head.as_ref().unwrap().value == 1);
        assert!(list.len == 1);

        list.insert_at_tail(2);
        list.insert_at_tail(4);
        list.insert_at_ith(1, 2);

        assert!(list.len == 4);
    }

    #[test]
    fn delete_head_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);

        list.delete_head();
        assert!(list.head.as_ref().unwrap().value == 2);
    }

    #[test]
    fn delete_tail_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);

        list.delete_tail();
        assert!(list.get_ith(1) == Some(&2));
        assert!(list.len == 2);
    }

    #[test]
    fn delete_ith_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);

        list.delete_ith(1);
        assert!(list.get_ith(1) == Some(&3));
    }

    #[test]
    fn get_ith_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);

        assert!(list.get_ith(1).is_some());
        assert!(list.get_ith(1) == Some(&2));
    }

    #[test]
    fn get_test() {
        let mut list: List<u32> = List::new();

        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);

        assert!(list.get().is_some());
        assert!(list.get() == Some(&1));
    }
}
