#![allow(unused_must_use)]
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn run(){
    let (transmitter, receiver) = mpsc::channel(); // creating new channel
     //new thread is created using thread::spawn() which takes an closure as argument
    let child_thread= thread::spawn(move|| {
      for i in 1..5 {
         println!("hi number {} from the spawned thread!", i);
         thread::sleep(Duration::from_secs(2));
      }
        transmitter.send(32); //move transfers ownership of transmitter to this thread.
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