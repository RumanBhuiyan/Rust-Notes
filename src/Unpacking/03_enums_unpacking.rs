#![allow(unused)]

#[derive(Debug)]
enum Types{
    I8(i8),
    I16(i16),
    I32(i32),
}

#[derive(Debug)]
enum Test{
    Name(String),
}

pub fn run() {
    let number = Types::I8(12);
    println!("Before unpacking number = {:?}", number);

    // Process - 01 : unpacking enum variants value using match expression
    let number  = match number {
        Types::I8(value) => value,
        _ => 0,
    };
    println!("After unpacking number = {:?}", number);


    // Process - 02 : using if let else statement
    let number2 = Types::I8(30);
    let mut result : i8 = 0;
    if let Types::I8(value) = number2{
        result = value;
        println!("result from inner= {:?}",result);
    }
    println!("result from outer = {:?}",result);

    let result = if let Types::I8(value) = number2 { value } else { 0 };
    println!("result= {:?}",result);

    // Process - 03 : using pattern matching unpacking technique for single type enum
    let test = Test::Name("Roman".to_string());
    let Test::Name(name) = test;
    println!("Name = {}", name);
}