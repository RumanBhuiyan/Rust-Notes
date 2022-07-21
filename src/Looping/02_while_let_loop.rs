#![allow(unused)]

pub fn run(){
    // Type - 02 : while let loop
    let mut numbers = vec![1,2,3,4,5];

    while let Some(value) = numbers.pop(){
        println!("{}", value);
    }
}
