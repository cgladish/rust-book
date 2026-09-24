use std::iter::FromIterator;

fn main() {
    let mut list = SinglyLinkedList::new();
    
    list.insert_head(32);
    list.insert_head(65);
    list.insert_head(10);

    let head_value = list.head.as_deref().unwrap().value;
    println!("Head: {head_value}");

    for val in list.iter() {
        println!("{val}");
    }
    for val in list.iter() {
        println!("{val}");
    }
}

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct SinglyLinkedList {
    head: Option<Box<Node>>
}
impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    fn insert_head(&mut self, value: i32) {
        let node = Node {
            value,
            next: self.head.take()
        };
        self.head = Some(Box::new(node));
    }

    fn iter(&self) -> SinglyLinkedListIter<'_> {
        SinglyLinkedListIter {
            next: self.head.as_deref()
        }
    }
}

struct SinglyLinkedListIter<'a> {
    next: Option<&'a Node>
}
impl<'a> Iterator for SinglyLinkedListIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(node.value)
    }
}