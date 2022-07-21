#![allow(unused)]

pub fn run(){
    // Type - 03 : loop
    let mut counter = 1;

    loop {
        println!("Counter : {}", counter);
        counter += 1;

        if counter > 5{
            break
        }
    }
}