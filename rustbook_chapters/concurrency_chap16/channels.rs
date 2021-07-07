use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();

    // * we can clone the sending channel to have multiple senders 
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val1 = vec!["Hello", "Ferris"];

        for val in val1 {
            tx1.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }

    // * We create two threads that send data to the main thread 
    // * Main thread doesn't end because the for loop will continue until all senders are closed
}