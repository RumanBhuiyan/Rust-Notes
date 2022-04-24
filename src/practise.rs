#![allow(unused, dead_code)]
use std::io::*;
/*
    In unsafe block of Rust you can tell Rust not to be worried about 4 rules of below.
    I have taken care of them please Rust don't check them. Those are
     1. Dereference a Raw pointer
     2. Call an unsafe function or method
     3. Access or modify a static variable
     4. Implement an unsafe Trait
 */
static  mut PI : f32 = 3.1446;

pub fn run(){
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // Rules - 01 (you are not allowed two execute 2 lines below outside unsafe block)
        println!("De-referencing r1 = {}",*r1);
        println!("De-referencing r2 = {}",*r2);

        // Rules - 02:
        get_name();

        // Rules - 03
        PI = 2.52;
        println!("After modification of static/global variable PI = {}",PI);
    }
}

unsafe fn get_name(){}

pub fn get_input() -> i32 {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}