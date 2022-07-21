#![allow(unused)]

use std::io::{stdin};

trait CheckPositive{
    fn is_positive(&self) -> bool;
}

impl CheckPositive for f32{
    fn is_positive(&self) -> bool {
        self >= &0f32
    }
}

trait HasSqrt{
    fn get_sqrt(&self) -> f32;
}

impl HasSqrt for f32{
    fn get_sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }
}

fn root_of<Number>(num : &Number) -> Option<f32> where Number : CheckPositive + HasSqrt{
    if num.is_positive(){
        Some(num.get_sqrt())
    }else{
        None
    }
}

pub fn run(){
    let num1 = 100f32;
    let num2 = -25f32;
    println!("root of {} : {:?}", num1, root_of(&num1));
    println!("root of {} : {:?}", num2, root_of(&num2));
}
