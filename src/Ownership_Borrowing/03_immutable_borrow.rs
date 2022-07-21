#![allow(unused)]

use std::io::{stdin};

pub fn run(){
    let numbers = vec![1,2,3,4,5];

    // Process - 01 : immutable borrow
    let mut index = 0;
    for num in &numbers{
        index += 1;
        println!("index : {} value : {}", index, *num);
    }

    // Process - 02: immutable borrow inside a function
    show_elements(&numbers);
}

pub fn show_elements(nums : &Vec<i32>) {
    for num in nums{
        println!("{}", *num);
    }
}