use add_one;
use std::{ops::Deref, rc::Rc};

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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // it's enable because of dereference coercion.

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&e));
    let f = Cons(3, Rc::clone(&e));
    println!("count after creating b = {}", Rc::strong_count(&e));
    {
        let g = Cons(4, Rc::clone(&e));
        println!("count after creating c = {}", Rc::strong_count(&e));
    }
    println!("count after g goes out of scope = {}", Rc::strong_count(&e));

}
