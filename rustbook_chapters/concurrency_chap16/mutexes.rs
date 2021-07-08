
// * We saw previously the better practice of sening information, but now we can use Mutex to share memory as an alternative
// * With Rust oqnership system, it is much safer to share memory than in other languages

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use std::sync::Mutex;
use std::sync::Arc;
use std::rc::Rc;

fn main() {

    // Mutex allows only one thread to access at a time, so
    // thread must ask to be able to access

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // * If we try this with threads we run into ownership issues

    let counter = Mutex::new(1);

    thread::spawn(move || {

        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
        
    });
    // ! Can't have another thread that takes and moves counter,
    // ! its an ownership issue


    // ! Also can't use Rc<T> because it doesn't account for changes from concurrency 
    // let counter1 = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {

    //     let counter1 = Rc::clone(&counter1);
    //     let handle = thread::spawn(move || {
    //         let mut theNum = counter1.lock().unwrap();

    //         *theNum += 1; 
    //     })
    //     handles.push(handle);
    // }

    // for h in handles {
    //     h.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());



    // We can use atomics though, which add this safety

    let counter1 = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {

        let counter1 = Arc::clone(&counter1);
        let handle = thread::spawn(move || {
            let mut theNum = counter1.lock().unwrap();

            *theNum += 1; 
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter1.lock().unwrap());


    // Last note:
    // Sync and Send Traits are part of the language and are empty traits aka only mark what is safe to send and share concurrently
    // These traits aren't impl for Rc<T> which thorws the erryr
    // Theses traits can be impl in own work when creating types to allow sharing and sending

}