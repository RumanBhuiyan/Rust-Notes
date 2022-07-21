#![allow(unused)]

/*
    Creating String
 */

pub fn run(){
    // Process - 01 : using to_string()
    let country = "Bangladesh".to_string();

    // Process - 02 : using to_owned()
    let country = "Bangladesh".to_owned();

    // Process - 03 : using format!()
    let country = format!("{}","Bangladesh");

    // Process - 04 : using String::from() or String::new()
    let country = String::from("Bangladesh");
    let mut country2 = String::new();
    country2.push_str("Banglades");
    country2.push('h');
    println!("country : {} country2 : {}", country, country2);
}