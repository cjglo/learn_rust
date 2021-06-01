// building a cons list to use Box pointer in way that is relevant for

use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>), // change Rc Ref to Rc to Box and back for different parts of code
    Nil,
}

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

    // The rest of the section is in functions
    
    // ref_counted_smart_ptr();

     

    // * NOTE: Interior mutability is accomplished through RefCell<T> 
    // * This allows to have borrow rules checked at runtime 
    // * This is used for when compiler can't tell code is safe, so this allows 
    // * us to pass immatuble variables but have them be mutable.
    // * but you still MUST follow the borrow rules, otherwise run time crash


    // ! using Rc<RefCell<T>> together lets you have multiple owners and mutaute!
    // ! it is very cool because the RefCell keeps track of references and protects us from data races!
    println!("\n\n");
    example_rc_ref();


//
}



fn ref_counted_smart_ptr() {

    // ! Commented out because needs List to be i32, rc<List>

    // // * For when a single value may have multiple ownership
    // enum List {
    //     Cons(i32, Rc<List>),
    //     Nil
    // }

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a)); 

    // /* 
    // Rc clone increases reference count, so a will only be cleaned up
    // references to it are out of scope

    // Important to note: Rc::clone is NOT a deep copy
    // */
    // println!("Strong count of references:{}", Rc::strong_count(&a));
    // {
    //     let d =  Cons(5, Rc::clone(&a));
    //     println!("Strong count of references:{}", Rc::strong_count(&a));
    // }
    


}


fn example_rc_ref() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    *value.borrow_mut() += 10; // recursive deref allows one * to cycle until hit 5, this is from previous section

    println!("a after: {:?}", a);
    println!("b after: {:?}", b);

    // ! HUGE NOTE: IT IS POSSIBLE TO MEMORY LEAK WITH THIS!!!! 
    // ! if you have refernces to eachother, then ref counter never hits 0 and so
    // ! memory is NEVER cleaned! So be careful with Rc<RefCell<T>>!!!!

}

// * We continue this with Weak<T> which is a weak reference that helps prevent the above memory leak issue
// * THis will be done in tree folder
