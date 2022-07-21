#![allow(unused)]

#[derive(Debug)] // derive fmt method of std::fmt::Debug trait to use for {} placeholder
pub struct Person{
    name : String,
    age : i8,
}

/*
    Or we can define a trait of types and methods that should be implemented by any struct
    like below

    N.B => Rust doesn't allow to define primitive types with method parameters inside trait rather
             you will have to define your custom type and use them. At the time of implementing trait
            you should also define those types.
 */
trait PersonFuncs{
    type NameType;
    type AgeType;

    fn get_name(&self) -> String;
    fn set_name(&mut self, value : Self::NameType);
    fn get_age(&self) -> i8;
    fn set_age(&mut self,value : Self::AgeType);
}

impl PersonFuncs for Person{
    type  NameType = String;
    type AgeType = i8;

    fn get_name(&self) -> String{
        String::from(&self.name)
    }
    fn set_name(&mut self, value : String){
        self.name = value
    }
    fn get_age(&self) -> i8{
        self.age
    }
    fn set_age(&mut self, value : i8){
        self.age = value
    }
}

pub fn main() {
    let mut p1 = Person{
        name : "Alex".to_string(),
        age : 24,
    };

    // Accessing getter methods
    println!("name : {}", p1.get_name());
    println!("age  : {}", p1.get_age());

    // Accessing setter methods
    p1.set_name("Thomas".to_string());
    p1.set_age(26);

    println!("name : {}", p1.get_name());
    println!("age  : {}", p1.get_age());
}