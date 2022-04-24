use std::collections::HashMap;

pub fn run(){
    let mut numbers = HashMap::new();

    numbers.insert(String::from("One"), 1);
    numbers.insert(String::from("Two"), 2);

    dbg!(&numbers);
    dbg!(&numbers["One"]);
}