#![allow(unused)]

/*
    As array is stack allocated resources so it's size must be known at compile time not in runtime.
    That's why we can't use variable as size in array. But we can use const variables.
 */

pub fn run(){
    // Process - 01
    let k = [true; 5];
    dbg!(&k);

    // Process - 02
    const SIZE: usize = 5;
    let k = [true; SIZE];
    dbg!(&k);
}