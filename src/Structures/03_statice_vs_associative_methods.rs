#![allow(unused)]

pub struct Person{
    name : String,
    age : i8,
}

/*
    Static methods are called Associative methods in Rust.

    The methods which are embedded with struct not with struct instances those are called
    Associative methods. The method below, you can access it like Person::show_info because
    show_info is embedded with Person struct. You can't call it like p1.show_info().you know why.
 */
impl Person{
    fn show_info(p : &Person){
        println!("Person's name : {} age : {}", p.name, p.age);
    }
}

pub fn main() {
   let p1 = Person{
        name : "Alex".to_string(),
        age : 24,
   };

    Person::show_info(&p1);
}