
use std::thread;
use std::time::Duration;

fn main() {

    // spawn new threads with thread::spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        } 
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // if we leave just the above code, no guarentee the spawn thread is run 
    // since all ends when main thread ends, so need this to make sure they are run:
    handle.join().unwrap(); // blocks other thread from exiting 


    // if you move line 22 before the for loops, the main thread will wait until the new one is done before continueing at all
}