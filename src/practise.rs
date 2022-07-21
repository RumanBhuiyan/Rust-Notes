#![allow(unused)]

use std::io::{stdin};

pub fn run(){
    //
    let mut numbers = Vec::with_capacity(1000);

}

pub fn  get_int_input() -> i32{
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}

pub fn get_line_input() -> String{
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}
