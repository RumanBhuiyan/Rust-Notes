#![allow(unused)]

use std::sync::Arc;

pub fn run(){
    let v1 = Arc::new(vec![1,2,3,4,5]);
    let v2 = Arc::clone(&v1); // cloning using functional approach syntax
    let v3 = v1.clone(); // cloning using object oriented style syntax

    println!("v1 : {:?}", v1);
    drop(v1); // Telling Rust to drop v1

    println!("v2 : {:?}", v2); // As v2 is the another owner of vector so we can access it by v2
    drop(v2);

    println!("v3 : {:?}", v3); // As v3 is the another owner of vector so we can access it by v3
}