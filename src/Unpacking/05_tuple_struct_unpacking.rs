#[derive(Debug)]
struct Person(
    String,
    i8,
    String
);

pub fn run() {
    let person1 = Person("Roman".to_string(), 24, "Software Engineer".to_string());
    let person2 = Person("Roman".to_string(), 24, "Software Engineer".to_string());
    dbg!(&person1);

    // Process -01 : tuple-struct unpacking using pattern matching technique
    let Person(name, age, occupation) = person1;
    println!("name : {} age : {} occupation : {}", name, age, occupation);

    // // Process - 02 : tuple-struct unpacking using conventional match statement
    match person2{
        Person(name, age, occupation) =>  {
            println!("name : {} age : {} occupation : {}", name, age, occupation)
        }
    }
}