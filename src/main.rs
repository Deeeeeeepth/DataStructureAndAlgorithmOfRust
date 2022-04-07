fn main() {
    println!("Hello, world!");
}
struct List<T> {
    head: Option<Node<T>>,
    len: u32,
}

struct Node<T> {
    value: T,
    next: Box<Node<T>>,
}

impl<T> List<T> {
    fn insert_at_head() {}
    fn insert_at_tail() {}
    fn insert_at_ith() {}
    fn delete_head() {}
    fn delete_tail() {}
    fn delete_ith() {}
    fn get() {}
    fn get_ith() {}
}
