#![allow(unused)]

pub fn run(){
    let mut numbers = [3,5,1,8,-1];

    println!("Summation : {}", numbers.iter().sum::<i32>());
    println!("Product :  {}", numbers.iter().product::<i32>());
}