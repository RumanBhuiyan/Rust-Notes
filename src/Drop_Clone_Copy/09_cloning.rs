#![allow(unused)]

/*
    Some types come with default clone() method like vector. In these cases we don't need to
    implement our own clone method of Clone trait rather we can use default one.

    But in some types like enum, struct, tuple struct there is no default clone implementation.
    So to clone objects of them we must implement clone method of Clone trait or use
    #[derive(Clone)]

    N.B => Cloning provides us a deep copy of real object but keep in mind that in some cases
            cloning may be time consuming where you will have to pay run time cost.
 */

// Process - 01 : cloning
#[derive(Debug, Clone)]
struct S(i32);

// Process - 01 : cloning
#[derive(Debug)]
struct P(f32);

impl Clone for P{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

pub fn run(){
    // Example - 01
    let v1 = vec![1,2,3,4,5];
    let v2 = v1.clone();
    println!("v1 : {:?}", v1);
    println!("v2 : {:?}", v2);

    let mut s1 = S(20);
    let mut s2 = s1.clone();
    println!("Before changing s1 : {:?}", s1);
    println!("Before changing s2 : {:?}", s2);

    // As cloning is deep of any object so any changes to one object wont be reflected in another
    s1.0 = 1;
    s2.0= 2;
    println!("After changing s1 : {:?}", s1);
    println!("After changing s2 : {:?}", s2);

    // Example - 02
    let p1= P(2f32);
    let p2 = p1.clone();
    println!("p1 : {:?}", p1);
    println!("p2 : {:?}", p2);
}