#![allow(unused)]

use std::env::{set_var,var, vars};


pub fn run(){
    // Showing existing environment variables
    for (key,value) in vars(){
        println!("key : {} value : {}",key, value)
    }

    // Set an temporary environment variables which will gone when program execution is finished
    // N.B => Key must be String , char type key isn't allowed
    set_var("R", "Ruman");

    // Accessing any environment variable's value by Key
    println!("key : {} value : {}", "R", var("R").unwrap());
}