pub fn run(){
     // Process - 01 : consuming Option enums value using match statement
    let response = get_something('s');
    match response {
        Some(letter) => println!("letter is {}", letter),
        None => println!("You sent any letter except s")
    }

    // Process - 02 : consuming Options enums value using unwrap()
    println!("letter : {}", get_something('s').unwrap());

    // Process - 03 : built-in functions for detecting Some() or None variant
    println!("is_some() : {}", get_something('s').is_some());
    println!("is_none() : {}", get_something('s').is_none());
}

// None doesn't require any type so just specify the type for Some() variant
pub fn get_something(letter : char) -> Option<char>{
    if letter == 's'{
        Some(letter)
    }else{
        None
    }
}