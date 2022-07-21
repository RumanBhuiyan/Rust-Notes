#![allow(unused)]

struct S(i32);

/*
    drop method of Drop trait is called automatically when execution comes out of a scope.
    If we don't implement drop method of Drop trait then Rust calls built-in drop methods
    otherwise it calls our implementation.

    N.B => Dropping values from last to first. That means regular objects are dropped reversely.
 */

impl  Drop for S{
    fn drop(&mut self) {
        println!("Dropping {}",self.0);
    }
}

pub fn main(){
    let a = S(1);
    let b = S(2);
    let c = S(3);
    {
        let d = S(4);
        let e = S(5);
        let f = S(6);
        println!("Inner....")
    }
    println!("Outer....")
}
// Output :
// Inner....
// Dropping 6
// Dropping 5
// Dropping 4
// Outer....
// Dropping 3
// Dropping 2
// Dropping 1