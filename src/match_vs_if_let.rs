/*
    If let pattern can don anything that match expression can do. If let is a shorthand version of
    match expression which we use just to handle one pattern instead of handling multiple pattern
    in match expression.
 */

#![allow(unused)]
pub fn run(){
    let keep = Some(5);
    let keep2 = None;

    // retrieving value from Some() variant
    // Process - 01:
    println!("value inside Keep : {}",get_value(keep));
    println!("value inside Keep2 : {}",get_value(keep2));

    // process - 02 : unwrap() returns the value of Some() variant but panics at None
    // println!("{}",keep.unwrap());
    // println!("{}",keep2.unwrap());

    // in some cases match statements can be verbose where we can use if let expression
    if let Some(value) = keep {
        println!("value inside Keep : {}",value);
    }else {
        println!("value inside Keep2 : {}",0);
    }
}

pub fn get_value(data : Option<i32>) -> i32{
    match data {
        Some(value) => value,
        None => 0
    }
}

