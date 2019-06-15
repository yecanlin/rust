enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    //let a = Cons(10,
    //    Box::new(Cons(5,
    //        Box::new(Nil))));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after ceating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after ceating b  = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("count after ceating c = {}", Rc::strong_count(&a));
    {
        let d = Cons(2, Rc::clone(&a));
        println!("count after ceating d = {}", Rc::strong_count(&a));
    }
    println!("count after d goes out of scope = {}", Rc::strong_count(&a));
    let e = Rc::new(9);
    //let f = vec![3, Rc::clone(&e)];
    let f = Rc::clone(&e);
    println!("f value is: {}", f);
}
