#![allow(unused)]

/*
     Objects that aren't copyable have to be cloned. Cloning is nothing but deep copy of any object.

     Below person isn't copyable as name is String type and we know that String is heap allocated
     and it has no copy trait. So you can't make it copyable by #[derive(Copy)] or
     impl Copy for Person { fn  copy(){} }.

     N.B => by #[derive(Copy)] we tell Rust to make it copyable for us
            by #[derive(Clone)] we tell Rust to make it cloneable for us

     As Person isn't copyable so you will have to clone it
*/

// Process - 01 : cloning
#[derive(Debug, Clone)]
struct Person{
    name : String,
    age : i8,
}

// Process - 02 : cloning
#[derive(Debug)]
struct Student{
    name : String,
    roll_number : i32,
}

impl Clone for Student {
    fn clone(&self) -> Self {
        Self{
            name : String::from(&self.name),
            roll_number : self.roll_number,
        }
    }
}

pub fn run(){
    // Example - 01
    let person1 = Person{name : "Alex".to_string(), age : 24};
    println!("person1 : {:?}",person1);

    // cloning (deep copy) of person1
    let person2 = person1.clone();
    println!("person2 : {:?}",person2);

    // Example - 02
    let student1 = Student{name : "A".to_string(), roll_number: 1};
    let student2 = student1.clone();
    println!("student1 : {:?}",student1);
    println!("student2 : {:?}",student2);
}