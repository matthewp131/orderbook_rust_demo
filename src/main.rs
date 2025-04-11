use std::thread;
use std::sync::{mpsc, Arc};
use std::sync::Mutex;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let input_thread = thread::spawn(move || {
        for _ in 0..10 {
            tx.send("hello").unwrap();
        }
    });

    let output_thread = thread::spawn(move || {
        for x in rx {
            println!("{x}");
        }
    });

    input_thread.join().unwrap();
    output_thread.join().unwrap();
}