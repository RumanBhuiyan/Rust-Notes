#![allow(dead_code)]
pub fn run(){
    println!("{} {}",1,2);
    println!("{1} {0}",1,2);//argument by index
    println!("{first} {second}",first=1,second=2);//argument by name

    let ruman = Person {
        first_name : String::from("Ruman"),
        last_name:String::from("Bhuiyan")
    };
    println!("person : {:?}",ruman);
    println!("Person : {:#?}",ruman);
    dbg!(ruman); // dbg!() is actually {:#?} under the hood
}

// this allows to print struct type instance with {:?}
#[derive(Debug)]
struct  Person{
    first_name : String,
    last_name : String
}
