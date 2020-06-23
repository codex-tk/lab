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

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop MyBox");
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::smart_ptrs::ListX::Nil;

enum List0{
    Cons(i32, Rc<List0>),
    Nil,
}

enum ListX{
    Cons(i32, RefCell<Rc<ListX>>),
    Nil,
}

impl ListX {
    fn tail(&self) -> Option<&RefCell<Rc<ListX>>> {
        match *self {
            ListX::Cons(_, ref list) => Some(list),
            Nil => None,
        }
    }
}

pub fn main(){
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3, Box::new(List::Nil))))));
    let v = 10;
    let rv = &v;
    let rrv = &rv;
    println!("{} {} {}", rrv,  *rrv, **rrv);

    let mbox = MyBox::new(32);
    println!("{}", *mbox);
    std::mem::drop(mbox);
    let mbox = MyBox::new(String::from("Hello"));
    hello(&mbox); // deref coercion &MyBox<String> -deref-> &String -deref-> &str
    hello(&(*mbox)[..]);
    std::mem::drop(mbox);


    let a = Rc::new(List0::Cons(5, Rc::new(List0::Cons(10, Rc::new(List0::Nil)))));
    let b = List0::Cons(3, Rc::clone(&a));
    let c = List0::Cons(5, Rc::clone(&a));

    let list = ListX::Cons(0, RefCell::new(Rc::new(Nil)));

    if let Some(tail) = list.tail() {
        *tail.borrow_mut() = Rc::new(ListX::Nil);
    }

}

