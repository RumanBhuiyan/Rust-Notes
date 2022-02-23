use random_number as rand; // give a custom name to your import with as keyword
use random_number::{random}; // import the only thing that you need instead of importing all

// random-number = "0.1.6"   add this line to toml file as a dependency of this project
pub fn run(){
    let number : u8 = random!();
    let number2 : i32 = rand::random!(1..10);
    dbg!(number);
    dbg!(number2);
}