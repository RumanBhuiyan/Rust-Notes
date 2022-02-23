use std::env::args;

pub fn run(){
    let numbers : Vec<String> = args().collect();
    dbg!(&numbers);
    println!("Summation of numbers : {}",get_sum(&numbers[1..numbers.len()].to_vec()));
}

pub fn get_sum(numbers : &Vec<String>) -> i32 {
    let mut summation  = 0;
    for num in numbers.iter(){
        summation += num.parse::<i32>().unwrap();
    }
    summation
}