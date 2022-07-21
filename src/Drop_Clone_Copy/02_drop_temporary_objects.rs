#![allow(unused)]

struct S(i32);

/*
    Here _ is temporary objects and temporary objects are deallocated when ; is found by Rust.
    So we can say that, temporary objects are dropped immediately after ; in each line end.
 */

impl  Drop for S{
    fn drop(&mut self) {
        println!("Dropping {}",self.0);
    }
}

pub fn main(){
    let _ = S(1);
    let _ = S(2);
    let _ = S(3);
    {
        let _ = S(4);
        let _ = S(5);
        let _ = S(6);
        println!("Inner....")
    }
    println!("Outer....")
}
// Output :
// Dropping 1
// Dropping 2
// Dropping 3
// Dropping 4
// Dropping 5
// Dropping 6
// Inner....
// Outer....