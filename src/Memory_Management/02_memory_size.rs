//use std::mem::* ;// * imports everything inside mem submodule but you don't need all of them
use std::mem::{size_of, size_of_val}; // so the best practise is to import the only thing that you need

pub fn main(){

    let a = 45;

    // knowing memory taken by any resource
    println!("memory taken by a = {:?}", std::mem::size_of_val(&a));
    println!("memory taken by i32 = {:?}", std::mem::size_of::<i32>());

    println!("memory taken by a = {:?}", size_of_val(&a));
    println!("memory taken by i32 = {:?}", size_of::<i32>());

}