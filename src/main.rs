use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a's initial rc count is: {}", Rc::strong_count(&a));
    println!("a's next item: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a's rc count after b is created is: {}", Rc::strong_count(&a));
    println!("b's initial rc count is: {}", Rc::strong_count(&b));
    println!("b's next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a: {}", Rc::strong_count(&b));
    println!("a rc count after changing b: {}", Rc::strong_count(&a));

    // this line creates a cycle and overflows the stack
    // println!("a next item: {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![Rc::clone(&leaf)]),:

    });
}

