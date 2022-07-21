/*
    enumerate() takes an iterator and returns a tuple of (index, value) in each iteration.
        As usual as numbers isn't an iterator itself but enumerate() requires an iterator that's
        why we have to create an iterator over numbers by numbers.iter() prior to enumerate()
*/

pub fn run() {
    let numbers = vec![1,2,3,4,5];

    for (index, value) in numbers.iter().enumerate(){
        println!("index : {} value : {}", index, value);
    }
}