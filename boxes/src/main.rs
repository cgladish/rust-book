use std::iter::FromIterator;
use std::ops::Deref;

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


    let custom_boxed_str = MyBox(String::from("chika chika Slim Shady"));
    hello(&custom_boxed_str); // Double deref coercion

    std::mem::drop(custom_boxed_str);

    println!("Goodbye!");
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

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("I'm going away forever!");
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
