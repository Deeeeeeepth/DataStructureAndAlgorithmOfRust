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
    fn new() -> List<T> {
        List { head: None, len: 0 }
    }
    fn insert_at_head(mut self, value: T) -> List<T> {
        let mut node = Node::new(value);
        node.next = self.head;
        self.head = Some(Box::new(node));
        self.len += 1;
        self
    }
    fn insert_at_tail(&mut self, value: T) {}
    fn insert_at_ith() {}
    fn delete_head() {}
    fn delete_tail() {}
    fn delete_ith() {}
    fn get() {}
    fn get_ith() {}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_at_head_test() {
        let list: List<u32> = List::new();

        let list = list.insert_at_head(1);
        assert!(list.head.unwrap().value == 1);
        assert!(list.len == 1);
    }
}
