/*
    when programmers tries to access the memory address directly by &b or &&b then rust
    compiler doesn't allow it rather it performs auto de-referencing then returns the value
 */
#![allow(unused)]

pub fn run(){
    let b = &&5;

    println!("b = {}", b); // 5 ; b is memory address so returns value by auto de-referencing

    println!("&b = {}", &b); // 5 ; &b is memory address so returns value by auto de-referencing
    println!("&&b = {}", &&b); // 5 ; &&b is memory address so returns value by auto de-referencing

    println!("*b = {}", *b); // 5 ; *b = &5, compiler returns 5 after auto de-referencing &5
    println!("**b = {}", **b); // 5 ; *b = &5, then **b = 5

    println!("*&b = {}", *&b); // 5 ; &b = &&5 then *&b = *&&5 . compiler returns 5
    println!("&*b = {}", &*b); // 5 ; *b = 5 then &*b = &5 . compiler returns 5
}