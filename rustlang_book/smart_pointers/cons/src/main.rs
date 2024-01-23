use crate::List::{Cons,Nil};

pub enum List<T> {
   Cons(T, Box<List<T>>),
   Nil, 
}

fn main() {
    let _list = Cons(20, Box::new(Cons(10, Box::new(Nil))));
}
