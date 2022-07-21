#![allow(unused)]

/*
   copying is done automatically for stack allocated resources like below. That means copying is
    implicit by default

    cloning can be done by .clone() method which is explicit. That means rust doesn't perform
     cloning automatically rather we have to tell rust explicitly to clone any resources.
     cloning is a deep copy of any resources.
 */

pub fn main(){
    let a = 5;
    let b = a; // copying
    let c = a.clone(); // cloning

    println!("a : {} b : {} c : {}",a , b, c);
}