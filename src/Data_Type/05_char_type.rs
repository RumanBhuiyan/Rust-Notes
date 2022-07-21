#![allow(unused)]

use std::char;

/*
    There are lots of pre-built methods available in std::char; Please explore them by yourself.
 */

pub fn run(){
    // converting character to digit
    println!("{:?}",char::to_digit('5', 10).unwrap());
    println!("Is a alphabetic ?: {}", char::is_alphabetic('a'));
}