#![allow(unused)]

use std::mem::swap;

/*
    () is called Zero tuple. This is the default return value of such functions where we don't
    specify functions return type explicitly. swap() function also returns zero tuple.
 */

pub fn run(){
    let mut a = 4;
    let mut b = 5;
    println!("before swapping : a = {} b = {}", a, b);

    let response = swap(&mut a, &mut b);
    println!("after swapping : a = {} b = {}", a, b);
    println!("response : {:?}", response);
}