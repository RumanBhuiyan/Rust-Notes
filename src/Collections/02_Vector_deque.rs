#![allow(unused)]

use std::io::{stdin};
use std::collections::{VecDeque};

pub fn run(){
    let mut numbers = VecDeque::new();

    numbers.push_back(10);
    numbers.push_back(20);
    numbers.push_front(100);

    dbg!(&numbers);
}