// building a cons list to use Box pointer in way that is relevant for

use crate::List::{Cons, Nil};
use std::ops::Deref;


enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {

    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3,
                Box::new(Nil))))));

    // * The defer trait is how you can have a type treated as a ref (overloads * oeprator)
    // * Here is a self made Box (except without heap alloc)
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

    let x = 5;
    let y = MyBox::new(x);
   
    println!("{} \n", *y);

    

    
}
