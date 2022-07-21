#![allow(unused)]

struct Person{
    name : String,
    age : i8,
}

trait PersonFunc{
    fn new(name : String, age : i8) -> Self;
    fn greet(&self) -> String;
}

impl PersonFunc for Person{
    fn new(name: String, age: i8) -> Self {
        Self{
            name,
            age,
        }
    }
    fn greet(&self) -> String {
        format!("Hello {}",self.name)
    }
}

pub fn main() {
    let p = Person::new("Roman".to_string(), 24);
    println!("{}", p.greet());
}