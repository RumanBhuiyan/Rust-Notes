// there are 2 types of strings in rust
// i) string literal or string slice (&str) , it takes memory from stack,its immutable
// ii) String object (String) , it takes memory from heap,its mutable and growable size
pub fn run(){
    let country = "Bangladesh"; // string slice or string literals
    let citizenship = String::from("Bangladeshi"); // string object

    // converting &str -> String
    // String::from() and String::new() both are used to create an String object in heap
    // only difference is from() takes argument but now() doesn't.but both of them are growable size
    let country2 = country.to_string();
    let country3 = country.to_owned();
    let country4 = String::from(country);
    println!("country : {} country2 : {} country3 : {} country4 : {}",country,country2,country3,country4);

    // converting String -> &str
    let citizenship2 = citizenship.as_str();
    println!("citizenship : {} citizenship2 : {}",citizenship,citizenship2);

    let mut name = String::new();
    name.push('R'); // push single character
    name.push_str("uman"); // push string

    // String concatenation using + or format!()
    println!("{}",String::from(&name) + " Bhuiyan"); // string concatenation
    let full_name = format!("{} Bhuiyan",&name);
    println!("Full name : {}",&full_name);

    // iterating over strings
    for character in full_name.chars() {
        println!("{}",character)
    }

    // split by whitespace
    println!("Enter numbers : ");
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    for number in string.trim().split_whitespace() {
        println!("{}",number);
    }

    // split basis on any character
    println!("Enter comma seperated numbers : ");
    let mut another = String::new();
    std::io::stdin().read_line(&mut another).unwrap();
    for number in another.trim().split(','){
        println!("{}",number);
    }
}