pub fn run(){
    // using String::new() we are telling compiler to take memory form buffer/heap
    let mut word = String::new();
    println!("Enter your number");

    // taking input may fail that's why using unwrap() we are telling that
    // if Ok() then store value to word otherwise throw error. here we aren't handling Err()
    std::io::stdin().read_line(&mut word).unwrap();
    // parse is converting word to i32 as mentioned in left. As it may fail that's why we used unwrap()
    let number :i32 = word.trim().parse().unwrap();

    // traditional conditional statements
    if number > 0 {
        // if any block has just one statement then it doesn't require any semicolon in end
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