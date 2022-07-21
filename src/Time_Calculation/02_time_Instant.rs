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
    // process : 02 using Instant of time
    let start = time::Instant::now();
    get_sum(1000000);
    println!("Program took {:?}",start.elapsed());
}