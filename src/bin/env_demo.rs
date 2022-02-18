use std::env;


enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let args: Vec<String> = env::args().collect();

    let b = Box::new(5);
    println!(" b= {:?}", b);

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3, Box::new(Nil))))));
    let s = String::from("teset");
    println!("{}",s);
    let mut v = Vec::new();
    v.push(s);
    println!("{:?}",v)
}