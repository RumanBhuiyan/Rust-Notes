use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

pub fn run(){
    // Arc => Atomic Reference Count, Arc is a smart pointer which allows us to create any resource
    // which may have multiple ownership
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for i in 1..=5{
        let  counter = Arc::clone(&counter);
        let handler = thread::spawn(move||{
            let mut number = counter.lock().unwrap();
            *number += 1;
            println!("thread : {} counter : {:?}",i, number);
            thread::sleep(Duration::from_secs(1));
        });
        handlers.push(handler);
    }

    for handler in handlers{
        handler.join().unwrap();
    }

    println!("Now counter from main thread is {:?}",counter.lock().unwrap());
}