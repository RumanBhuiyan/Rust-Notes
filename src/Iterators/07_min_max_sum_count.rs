pub fn run() {
    // example - 01
    println!("minimum in range (1..=5) : {:?}", (1..=5).min().unwrap());
    println!("maximum in range (1..=5) : {:?}", (1..=5).max().unwrap());
    println!("sum of range (1..=5) : {:?}", (1..=5).sum::<i32>());
    println!("Number of iterations in range (1..=5) : {:?}", (1..=5).count());

    // example - 02
    let numbers = vec![1,2,3,4,5];
    println!("minimum in numbers {:?} : {}", &numbers, numbers.iter().min().unwrap());
    println!("maximum in numbers{:?} : {}", &numbers, numbers.iter().max().unwrap());
    println!("sum of numbers {:?} : {}", &numbers, numbers.iter().sum::<i32>());
    println!("Number of iterations in {:?} : {}", &numbers, numbers.iter().count());
}