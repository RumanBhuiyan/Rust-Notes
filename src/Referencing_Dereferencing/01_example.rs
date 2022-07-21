/*
    & is called referencing operator, &a denotes the address of memory that holds the value of a
    * is called de-referencing operator, *a denotes the value that is kept in memory address a

    N.B =>
        in &a , a must be a value
        in *a, a must be a memory address
 */
#![allow(unused)]
pub fn run(){

    let a = 7;

    println!("a = {}",a);
    println!("&a = {}",&a);
    println!("*&a = {}",*&a);
    // println!("*a = {}",*a); // Its not valid as a is a integer value which can't be de-referenced
}