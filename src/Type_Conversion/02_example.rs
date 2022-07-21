pub fn run(){
    // Process - 01 : type conversion using as operator
    let num1 = 25;
    println!("num1 as i8 : {}", num1 as i8);
    println!("num1 as i64 : {}", num1 as i64);
    println!("num1 as i128 : {}", num1 as i128);
    println!("num1 as u8 : {}", num1 as u8);
    println!("num1 as u16 : {}", num1 as u16);
    println!("num1 as u32 : {}", num1 as u32);
    println!("num1 as u64 : {}", num1 as u64);
    println!("num1 as u128 : {}", num1 as u128);
    println!("num1 as f32 : {}", num1 as f32);
    println!("num1 as f64 : {}", num1 as f64);

    // Process - 02 : Using turbo fish syntax
    let string = "123";
    let number = string.parse::<i32>().unwrap();
    println!("String after converting to i32 : {}", number);
}