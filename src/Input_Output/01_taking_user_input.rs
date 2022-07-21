use std::io::{stdin};

pub fn run(){
    // Process - 01 : Reading input handling Ok(), Err() variant using match statement
    let mut line = String::new();
    match stdin().read_line(&mut line){
        Ok(bytes) => println!("{} bytes were read",bytes),
        Err(error) => println!("Error : {}",error)
    };

    // Process - 02 : Reading input using unwrap() which returns Ok() value otherwise panics!
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let number = line.trim().parse::<i32>().unwrap();
    println!("Entered number : {}", number);
}