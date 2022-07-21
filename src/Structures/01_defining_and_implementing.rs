#![allow(unused)]

#[derive(Debug)] // derive fmt method of std::fmt::Debug trait to use for {} placeholder
pub struct Person{
    name : String,
    age : i8,
}

/*
    we can take multiple impl blocks to define methods for struct instances or we can
        write all those methods in a single impl block

    N.B => &self is borrowing attributes and their values immutably and &mut self can borrow
            mutably. That's why we took &sef in getter methods and &mut self in setter methods.
 */
impl Person{
    pub fn set_name(&mut self, value : String){
        self.name = value
    }
    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }
}

impl Person{
    pub fn set_age(&mut self, value : i8){
        self.age = value
    }
    pub fn get_age(&self) -> i8{
        self.age
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
