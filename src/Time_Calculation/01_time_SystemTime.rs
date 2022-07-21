#![allow(unused)]

use std::time;

pub fn get_sum(n :i32) -> i128{
    let mut summation:i128 = 0;
    for i in 1..=n {
        summation += i as i128;
    }
    summation
}

pub fn main(){
    // process : 01 (using SystemTime)
    let start = time::SystemTime::now();
    get_sum(1000000);
    let end = time::SystemTime::now();
    println!("Program took {:?}",end.duration_since(start).unwrap());
}