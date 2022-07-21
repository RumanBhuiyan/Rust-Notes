/*
    any takes an iterator and checks if any of the items of the iterator pass the condition or not.
    all takes an iterator and checks if all of the items of the iterator pass the condition or not.
*/

pub fn run() {
    // example - 01
    println!("any even number within range (1..=5) :{:?}", (1..=5).any(|x| x%2 == 0));
    println!("all even numbers within range (1..=5) :{:?}", (1..=5).all(|x| x%2 == 0));

    // example - 02
    let numbers = vec![1,2,3,4,5];
    println!("any even numbers in numbers {:?} : {}", &numbers, numbers.iter().any(|x| x % 2 == 0));
    println!("all even numbers in numbers{:?} : {}", &numbers, numbers.iter().all(|x| x % 2 == 0));
}