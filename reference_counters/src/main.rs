use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::mem::drop;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let print_count = |event: &str| {
        let count_a = Rc::strong_count(&a);
        println!("Count after {event}: {count_a}");
    };

    print_count("creating a");

    let b = Cons(4, Rc::clone(&a));
    print_count("creating b");

    {
        let c = Cons(3, Rc::clone(&a));
        print_count("creating c");
    }

    print_count("c goes out of scope");

    drop(b);
    print_count("dropping b");
}

enum List {
    Cons(i32, Rc<List>),
    Nil
}