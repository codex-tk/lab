use crate::smart_ptrs::List::{Cons, Nil};
use std::ops::Deref;

enum List{
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
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

pub fn main(){
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3, Box::new(Nil))))));
    let v = 10;
    let rv = &v;
    let rrv = &rv;
    println!("{} {} {}", rrv,  *rrv, **rrv);
}