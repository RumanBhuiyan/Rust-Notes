#![allow(unused)]

use std::{f32, f64};

/*
    f32 is float and f64 is double equivalent in Rust comparing to C/C++.
 */

pub fn run(){
    let radius = 2.5f32;

    // Mathematical const values are available under std::f32::consts and std::f64::consts
    println!("PI value : {}", f32::consts::PI);
    println!("PI value : {}", f64::consts::PI);

    // Other operations
    println!("Square root of 100 : {}", f32::sqrt(100f32));
    println!("Square root of 100 : {}", f64::sqrt(100 as f64));

    // MIN, MAX, ceil(), floor()
    println!("Minimum value of f32 type : {}", f32::MIN);
    println!("Minimum value of f32 type : {}", f32::MAX);
    println!("ceil of 2.2 : {}", f32::ceil(2.2));
    println!("floor of 2.2 : {}", f32::floor(2.2));
}