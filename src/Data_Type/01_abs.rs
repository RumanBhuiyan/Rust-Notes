#![allow(unused)]

use std::io::{stdin};

/*
    Duck typing means if it quacks like a duck then it's a duck. If any function gets the arguments
        which satisfy functions requirement to perform its operation then that is called duck typing
 */

pub fn run(){
    // -2 can be of i8, i16, i32, i64, i128 so to convert into which type abs we have to specify
    // let number = -2;
    // println!("abs of {} : {}", number, number.abs());

    // Solution - 01 (specify type at the time of declaration)
    let number:i8 = -2;
    println!("abs of {} : {}", number, number.abs());

    // Solution - 02 (specify type at the time of conversion)
    let number = -2;
    println!("abs of {} : {}", number,i8::abs(number));
}