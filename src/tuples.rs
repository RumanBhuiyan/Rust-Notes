pub fn run() {
    // declaring tuple
    let person : (String,i8,String) = (String::from("Ruman"),24,String::from("Bangladeshi"));
    print_tuple_items(&person);
    dbg!(&person);
}

pub fn print_tuple_items(person : &(String,i8,String)) {
    // unpacking
    let (name,age,nationality) = person;
    println!("name : {} age : {} nationality : {}",name,age,nationality);

    // accessing tuple by index
    println!("name : {} age : {} nationality : {}",person.0,person.1,person.2);
}