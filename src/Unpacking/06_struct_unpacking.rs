#[derive(Debug)]
struct Person{
    name : String,
    age : i8,
    occupation : String,
}

pub fn run() {
    let person1 = Person{
        name : "Roman".to_string(),
        age : 24,
        occupation : "Software engineer".to_string(),
    };
    let person2 = Person{
        name : "ALex".to_string(),
        age : 28,
        occupation : "Software engineer".to_string(),
    };
    dbg!(&person1);

    // Process -01 : struct unpacking using pattern matching technique
    let Person { name, age, occupation} = person1;
    println!("Person 1 => name : {} age : {} occupation : {}", name, age, occupation);

    // // Process - 02 : tuple-struct unpacking using conventional match statement
    match person2{
        Person {name, age, occupation} =>  {
            println!("Person 2 => name : {} age : {} occupation : {}", name, age, occupation)
        }
    }
}