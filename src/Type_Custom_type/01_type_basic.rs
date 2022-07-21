#![allow(unused)]

pub fn main() {
    // Defining custom type
    type Point = f32;

    let mut p1 : Point = 4.5;
    let mut p2  = 2.5;

    // type conversions between primitive type to custom type and vice-versa
    p2 = p1 as f64; // custom type to primitive type (Point -> f64)
    p1 = p2 as Point; // primitive type to custom type (f64 -> Point)

    println!("p1 : {}", p1);
    println!("p2 : {}", p2);
}