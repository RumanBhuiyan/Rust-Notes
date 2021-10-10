fn main() {

    // integer data tyep
    // integer -: i8 , i16, i32, i64 , i128
    // unsigned integer : u8, u16, u32 , u64, u128
    let age = 24 ; // Rust will detect it as  i32 by default
    let digit : i8 = 5 ; // telling Rust to allocate 8 bit for integer
    let number : i32 = 180 ; // telling Rust to allocate 32 bit for integer
    
    println!("Age : {} digit : {} number : {}",age,digit,number);
    
    
    // float data type
    // float : f32 , f64
    let temperature = 33.5; // f64 by default for double precision
    let area :f32 = 80.5 ;
    let pi : f64 = 3.1416 ;
    
    println!("temperature : {} are : {} pi : {}",temperature,area,pi);
    
    // boolean data type
    let is_sunday = false; // detects it as boolean automatically
    let is_saturday : bool = true; // telling Rust to take it as boolean
    
    println!("isSunday : {} isSaturday : {}",is_sunday,is_saturday);
    
    // character type
    let keep = 'R';
    let keep_char : char = 'r' ;
    
    println!("keep : {} keep_char : {}",keep,keep_char);
    
    const USER_LIMIT:i32 = 100;    // Declare a integer constant
    const PI:f32 = 3.14;           //Declare a float constant

    println!("user limit is {}",USER_LIMIT);  //Display value of the constant
    println!("pi value is {}",PI);

    // Rust allows redeclaring variable but doesn't support auto-type cast
    let name : &str= "Ruman";
    println!("Name : {}",name);
        
    let name  = name.len();
    println!("name : {}",name);

    // string object
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());
    
}