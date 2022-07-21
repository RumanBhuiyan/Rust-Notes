#![allow(unused)]

use std::fmt::{Debug, Display, Formatter};
/*
    let's implement our custom fmt method of std::fmt::Display trait and std::fmt::Debug trait
        to show output in the way that we want instead of it's default look

    N.B => Here, f is formatter that will take your arguments and format them to print
            in terminal and this argument is must.
 */

struct Person{
    name : String,
    age : i32,
}

impl Display for Person{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Display : Person's name : {} age : {}",
            self.name,
            self.age
        )
    }
}

impl Debug for Person{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Debug : Person's name : {} age : {}",
            self.name,
            self.age
        )
    }
}

pub fn run() {
    let p1 = Person{
        name : "Alex".to_string(),
        age  : 23i32,
    };

    // consuming fmt method of Display trait using {}
    println!("{}", p1);

    // consuming fmt method of Debug trait {:?}
    println!("{:?}", p1);
    dbg!(&p1);
}