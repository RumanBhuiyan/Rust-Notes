#![allow(unused)]

/*
    There is no truthy value and falsy value concept in Rust. So you don't need to memorize any
    kind of truthy value or falsy value anymore. The condition used in conditional statement or
    in loop should generate boolean value otherwise Rust will throw error. Rust is very strict
    about that to remove ambiguity from code to others.
 */

pub fn run(){
    // converting boolean type to integer using as operator
    println!("true = {}",true as i32);
    println!("false = {}",false as i32);

    // expression evaluations
    let result = 2 > 4;
    println!("result : {}", result);
}