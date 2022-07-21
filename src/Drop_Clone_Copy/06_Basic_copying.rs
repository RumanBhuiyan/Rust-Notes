#![allow(unused)]

/*
    i32, f32 those primitive types are copyable that's why Rust has Copy trait implemented for them
    by default. But if you declare your own type using enum or struct etc and its fields are copyable
    though Rust wont implement Copy trait for them. That's why you will have to tell Rust explicitly
    to implement Copy trait for your type by #[derive(Copy)]. If your defined type has heap allocated
    resources like Vec, String that Copy will be failed as you know that Vec, String aren't copyable.
    Rater They are Cloneable so derive clone for them.

    Copy trait won't work if Clone trait isn't implemented. As Clone is super-trait of Copy trait,
    copy method will call clone method at the time of copying semantics

    N.B => copying will be failed if resources aren't copyable otherwise it will work fine.
            As Clone is super-trait of Copy you can use cloning technique both for copyable resources
            and not copyable resources without help of Copy but vice-versa isn't true. Rather you
            have to implement Clone for Copy.
 */

#[derive(Debug, Copy, Clone)]
struct Point(f32, f32);

/*
     Below person isn't copyable by clone super-trait. Rust can't do it for you automatically.
     So implement your own clone method for it.As name Field of Person struct is String and
     String has no Copy trait by default in Rust that's why rust can't do it for us automatically.
*/

// #[derive(Debug, Copy, Clone)]
// struct Person{
//     name : String,
//     age : i8,
//}

pub fn run(){
    // Example - 01
    let point1 = Point(2f32, 3f32);
    println!("point1 : {:?}", point1);

    // If we don't add #[derive(Copy, CLone)] then below copy operation will be failed
    let point2 = point1;
    println!("point1 : {:?}", point1);
    println!("point2 : {:?}", point2);
}