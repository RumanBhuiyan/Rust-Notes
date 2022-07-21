
pub fn run() {
    let person1 = ("Roman".to_string(), 24, "Software Engineer".to_string());
    let person2 = ("Roman".to_string(), 24, "Software Engineer".to_string());
    dbg!(&person1);

    // Process -01 : tuple unpacking using pattern matching technique
    let (name, age, occupation) = person1;
    println!("name : {} age : {} occupation : {}", name, age, occupation);

    // Process - 02 : tuple unpacking using conventional match statement
    match person2{
        (name, age, occupation) =>  {
            println!("name : {} age : {} occupation : {}", name, age, occupation)
        }
    }
}