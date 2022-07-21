#![allow(unused)]

#[derive(Debug)]
struct S(i32);

impl Clone for S{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

/*
    Copy trait won't work if Clone trait isn't implemented. As Clone is super-trait of Copy trait,
    copy method will will call clone method at the time of copying semantics. Look below

    N.B => copying will be failed if resources aren't copyable otherwise it will work fine.
*/

impl Copy for S{}

pub fn main(){
    let  s1 = S(20);
    let s2 = s1; // copy semantics; Copy trait is calling clone() method of its super-trait
    println!("s1 : {:?}", s1);
    println!("s2 : {:?}", s2);

}