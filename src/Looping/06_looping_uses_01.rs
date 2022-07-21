pub fn run(){
    println!("Factorial of 5 is : {}",get_factorial(5));
    println!("Factorial of 5  is : {}",get_factorial2(5));
    println!("Factorial of 5  is : {}",get_factorial3(5));

    // accessing index, value simultaneously
    let  numbers = vec![1,2,3,4,5];
    for (index, value) in numbers.iter().enumerate(){
        println!("index : {} value : {}",index, value);
    }
}

pub fn get_factorial(number : i32) -> i32 {
    let mut product = 1;
    for i in 2..=number{
        product *= i;
    }
    return product
}

pub fn get_factorial2(mut number : i32 ) -> i32 {
    let mut product = 1;
    while number > 1 {
        product *= number;
        number -= 1;
    }
    product
}

pub fn get_factorial3 (mut number : i32) -> i32 {
    let mut product = 1;
    loop {
        if number <=1 { break }
        product *= number;
        number -= 1;
    }
    return product
}