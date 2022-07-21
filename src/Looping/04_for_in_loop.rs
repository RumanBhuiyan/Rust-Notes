#![allow(unused)]

pub fn run(){
    // Type - 04 : for in loop
    let mut numbers = vec![1,2,3,4,5];

    for num in &numbers{
        println!("{}", *num);
    }
}