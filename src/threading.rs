#![allow(unused_must_use)]
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn run(){
    // create new channel
    let (transmitter, receiver) = mpsc::channel();

     // thread::spawn() creates new thread parallel to main thread
     //move transfers ownership of transmitter to this thread.
    let child_thread= thread::spawn(move|| {
      for i in 1..5 {
         println!("hi number {} from the spawned thread!", i);
         thread::sleep(Duration::from_secs(2));
      }
        transmitter.send(32);
   });

   //code executed by the main thread
   for i in 1..5 {
      println!("hi number {} from the main thread!", i);
      thread::sleep(Duration::from_secs(1));
   }

    // wait for child_thread until it finishes its execution
    child_thread.join().unwrap();
    println!("Message from child thread : {:?}",receiver.recv().unwrap()); // receiving data from channel
}