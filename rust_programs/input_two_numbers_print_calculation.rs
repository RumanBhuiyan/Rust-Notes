#![allow(unused, dead_code)]

use std::io::*;

pub fn run(){
    print!("Enter the sign : ");
    stdout().flush().unwrap(); // flush print!() statements into stdout immediately
    let mut sign = String::new();
    stdin().read_line(&mut sign).unwrap();
    let sign: char = sign.trim().parse().unwrap();

    print!("Enter first number : ");
    stdout().flush().unwrap();
    let first = get_input();

    print!("Enter first number : ");
    stdout().flush().unwrap();
    let second = get_input();

    match get_func(sign) {
        Some(func) => println!("{}", func(first, second)),
        None => println!("Sign unknown")
    }
}

pub fn get_func(sign : char) -> Option<fn(i32, i32) -> i32>{
    match sign {
        '+' => Some(|a : i32, b: i32| a + b),
        '-' => Some(|a : i32, b: i32| a - b),
        '*' => Some(|a : i32, b: i32| a * b),
        '%' => Some(|a : i32, b: i32| a % b),
        _ => None
    }
}

pub fn get_input() -> i32 {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}