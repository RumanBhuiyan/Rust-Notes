#![allow(unused)]

use std::io::{stdin};
use std::collections::{HashMap};

pub fn run(){
    let mut numbers = HashMap::new();

    numbers.insert("One", 1);
    numbers.insert("two", 2);

    for (key, value ) in numbers{
        println!("Key : {} value : {}", key, value);
    }
}