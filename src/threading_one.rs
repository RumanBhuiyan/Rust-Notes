use std::thread;
use std::time::Duration;

pub fn run(){
    let handler = thread::spawn(||{
        for i in 1..=10 {
            println!("child thread : {}",i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // wait for child thread execution completion before executing next statements of main thread
    // handler.join().unwrap();

    for i in 1..=5 {
        println!("Main thread : {}",i);
        thread::sleep(Duration::from_secs(1));
    }

    // telling main thread to wait until child thread finishes it's execution
    handler.join().unwrap();
}