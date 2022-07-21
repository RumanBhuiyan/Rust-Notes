#![allow(unused)]

struct S(i32);

impl  Drop for S{
    fn drop(&mut self) {
        println!("Dropping {}",self.0);
    }
}

pub fn main(){
    S(1);
    S(2);
    S(3);
    {
        S(4);
        S(5);
        S(6);
        println!("Inner....")
    }
    println!("Outer....")
}
// output
// Dropping 1
// Dropping 2
// Dropping 3
// Dropping 4
// Dropping 5
// Dropping 6
// Inner....
// Outer....