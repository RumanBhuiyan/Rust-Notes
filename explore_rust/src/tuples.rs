pub fn run() {
    // declaring tuple
    let person : (String,i8,String) = (String::from("Ruman"),24,String::from("Bangladeshi"));
    print_tuple_items(&person);// print_tuple() is borrowing person tuple
    dbg!(&person); // dbg!() is also borrowing person tuple
}

pub fn print_tuple_items(person : &(String,i8,String)) {
    // unpacking
    let (name,age,nationality) = person;
    println!("name : {} age : {} nationality : {}",name,age,nationality);
    // accessing tuple item in another way
    println!("name : {} age : {} nationality : {}",person.0,person.1,person.2);
}