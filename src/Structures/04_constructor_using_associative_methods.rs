#![allow(unused)]

#[derive(Debug)]
pub struct Person{
    name : String,
    age : i8,
}
/*
    If methods parameters name and struct fields names are similar then you can use the shorthand
    syntax. just mention the field names without type : value.

    N.B => Look I used Self in new_short() instead of Person because If i change the struct name to
            something else then i will need to change in both of place of new_short().
            As Self = Person struct so we can use Self here instead of Person.

    By convention, constructor taking no arguments are named as new and taking single or multiple
                    arguments are named as from. for example, String::new(), String::from("abcd")
 */
impl Person{
    // Associative method elaborate syntax
    fn new(name_value : String, age_value : i8) -> Person{
        Person{
            name : name_value,
            age : age_value
        }
    }
    // Associative method short-hand syntax; Here Self denotes Person struct
    fn new_short(name : String, age : i8) -> Self{
        Self{
            name,
            age,
        }
    }
}

pub fn main() {
    let p1 = Person::new("ALex".to_string(), 26);
    let p2 = Person::new_short("Thomas".to_string(), 28);

    println!("p1 : {:?}", p1);
    println!("p2 : {:?}", p2);
}
