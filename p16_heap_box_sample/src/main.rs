use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    let x = 5;
    let y = &x;

    println!("{:?}", assert_eq!(5, x));
    //println!("{:?}", assert_eq!(5, y));
    println!("{:?}", assert_eq!(5, *y));


    let a = 6;
    let b = Box::new(a);
    assert_eq!(6, a);
    //assert_eq!(6, b);
    assert_eq!(6, *b);

    let c = 7;
    let d = MyBox::new(c);
    assert_eq!(7, c);
    assert_eq!(7, *d);
    println!("c is {}", c);

    let e = String::from("aaaa");
    let f = e;
    //println!("e is {}", e);

    let g = MyBox::new(String::from("world"));
    hello(&g);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
