#![allow(unused)]

use std::rc::Rc;

/*
    We know that how python manages objects by reference count instead of creating new object in heap
    at the time of assignment of heap allocated resources. This scenario allows multiple owners
    in sense of Rust. We can achieve that behaviour in rust using RC(Reference Count),
    ARC(Atomic Reference Count). RC is faster than ARC but RC is used for single thread and
    ARC is used for data sharing within multiple threads.

    N.B => drop() is nothing but a simple function where we pass ownership of heap allocated resources
            and it get dropped. drop() works for heap allocated resources, not for primitive types
            who have copy trait. RC doesn't allow mutable resources.
 */

pub fn run(){
    let v1 = Rc::new(vec![1,2,3,4,5]);
    // Here Rc::clone(&v1) not creating a deep copy of v1 rather cloning pointer of v1 and
    // assigning to v2. Now vec![1,2,3,4,5] has 2 owner. v1, v2
    let v2 = Rc::clone(&v1); // cloning using functional approach syntax
    let v3 = v1.clone(); // cloning using object oriented style syntax

    println!("v1 : {:?}", v1);
    drop(v1); // Telling Rust to drop v1

    println!("v2 : {:?}", v2); // As v2 is the another owner of vector so we can access it by v2
    drop(v2);

    println!("v3 : {:?}", v3); // As v3 is the another owner of vector so we can access it by v3
}
