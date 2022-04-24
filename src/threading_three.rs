#![allow(unused, dead_code)]
use std::io::*;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn run(){
    // mpsc => multiple producer single consumer
    // that means multiple thread can send values but single thread can receive that values
    // creating channel using mpsc::channel() which returns transmitter, receiver
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    let handler = thread::spawn(move||{
        for i in 1..=5{
            tx2.send(i); // sending value to main thread from this child thread
            thread::sleep(Duration::from_secs(1));
        }
    });
    // if you move only tx2 into another thread and don't drop tx then your main thread will
    // wait for response from that thread who owned tx and hang on. so drop it like below.
    // drop(tx);

    // here tx will be moved into handler2 and dropped by this thread
    let handler2 = thread::spawn(move||{
         for i in 6..=10{
            tx.send(i); // sending value to main thread from this child thread
            thread::sleep(Duration::from_secs(1));
        }
    });

    // blocks execution of main thread and wait until get value from child thread
    let keep = rx.recv().unwrap();
    println!("keep : {}",keep);

    // for each response from child thread
    for value in rx {
        println!("rx : {}",value);
    }

    handler.join().unwrap();
    handler2.join().unwrap();
}