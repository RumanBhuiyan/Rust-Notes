#![allow(unused)]

/*
    N.B => {} or {:?} looks for fmt implementation of std::fmt::Display trait for the type
            that we pass. For i32, tuple etc types, fmt is implemented by default. But for struct
            type fmt isn't implemented so you will have to tell Rust explicitly to derive
            fmt implementation for you by #[derive(Debug)] or you can manually implement that.
 */

#[derive(Debug)]
struct Person{
    name : String,
    age : i32,
}

pub fn main() {
    let p1 = Person{
        name : "Alex".to_string(),
        age  : 23i32,
    };
    println!("{:?}", p1);
}