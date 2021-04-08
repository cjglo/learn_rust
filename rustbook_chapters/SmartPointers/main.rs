// building a cons list to use Box pointer in way that is relevant for

use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;


// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

fn main() {

    // let list = Cons(1, 
    //     Box::new(Cons(2, 
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));

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

    // * DerefMut trait is used to creat mutable reference

    // * Quick note about "deref cocercion":
    // Rust will take a type like &MyBox<String> in a function argument that reuqires &str and call deref repeatdly until it has &str
        // aka it calls deref over and over until it gets the type it needs
        // It will do this from const ref to const ref, mut ref to mut ref, or mut ref to const ref
        // cons tto mut will not happen because of the borrow checker rule of mut ref being only ref in existence
            // it can't guarentee that in that one specific situation, but all others dertef cocerion will work

    

    // * Second trait to impl is Drop
    // * This isn't necessary for our MyBox cause no alloc, but this will illustrate basic idea:
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Would dealloc memory at this point!");
        }
    } 

    // * Broke up the rest of the section into functions for each area
    
}



fn ref_counted_smart_ptr() {

    // * For when a single value may have multiple ownership
    enum List {
        Cons(i32, Rc<List>),
        Nil
    }

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a)); 

}