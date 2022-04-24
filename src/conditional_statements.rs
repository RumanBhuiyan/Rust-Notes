pub fn run(){
    // String::new() takes memory form buffer/heap
    let mut word = String::new();
    println!("Enter your number");

    // unwrap() returns Ok() variant otherwise panics
    std::io::stdin().read_line(&mut word).unwrap();

    // parse() converts word to i32 for type inference in left
    let number :i32 = word.trim().parse().unwrap();

    // traditional conditional statements
    if number > 0 {
        // 1 statement in 1 block or last statement doesn't need semicolon
        println!("Positive number")
    }else if number==0 {
        println!("Zero")
    }else {
        println!("Negative number")
    }

    // match statement which is equivalent to switch case in c/c++
    match number {
        number if number>0 => println!("Positive number"),
        number if number<0 => println!("Negative number"),
        _ => println!("Zero")
    }

    // other uses of match
    match number {
        1 => println!("You picked number within 1 and 9"),
        11..=20 => println!("You picked number within 11 and 20"),
        _ => println!("You picked number except range of 1-20")
    }
}