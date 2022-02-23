// explore built-in functions offered by VecDeque from intellisense
use std::collections::VecDeque;

pub fn run(){
    let mut numbers = VecDeque::new();

    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_front(3);

    dbg!(&numbers);
    println!("Size : {}",numbers.len());

    for (index,value) in numbers.iter().enumerate() {
        println!("index : {} value : {}",index,value);
    }
}