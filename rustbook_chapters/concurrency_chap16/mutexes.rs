
// * We saw previously the better practice of sening information, but now we can use Mutex to share memory as an alternative
// * With Rust oqnership system, it is much safer to share memory than in other languages

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use std::sync::Mutex;

fn main() {

    // Mutex allows only one thread to access at a time, so
    // thread must ask to be able to access

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}