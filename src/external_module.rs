// random-number = "0.1.6"   add this line to project toml file
use random_number as rand; // giving custom name to importing module
use random_number::{random}; // retrieving exact module

pub fn run(){
    let number : u8 = random!();
    let number2 : i32 = rand::random!(1..10);
    dbg!(number);
    dbg!(number2);
}