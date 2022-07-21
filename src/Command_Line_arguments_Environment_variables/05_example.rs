use std::env::{args};

pub fn run(){
    // Here i am applying shadowing property of Rust in case of numbers variable
    let numbers : Vec<String> = args().collect();
    let numbers = numbers[1..].iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut summation = 0;
    for num in numbers{
        summation += num;
    }
    println!("Summation {}", summation);
}