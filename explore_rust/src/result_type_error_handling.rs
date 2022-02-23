// Errors are 2 types in Rust. i) Recoverable error ii) UnRecoverable error
// Recoverable errors are handled using Result<OK_type,Err_type>
pub fn run(){
    // use approach 1 when to deal with errors otherwise pick approach 2
    // process : 01 (handling error)
   match get_quotient(3 as f32,2 as f32){
       Ok(result) => println!("3/2 = {}",result),
       Err(e)=> println!("Err : {}",e)
   }
    // process : 02 unwrap() returns only Ok() value and panics when get Err()
    let result = get_quotient(3 as f32,2 as f32).unwrap();
    println!("3/2 = {}",result);

    // process : 03  expect() works like unwrap() but expect() can show a message before panic
    let result2 = get_quotient(3 as f32,2 as f32).expect("can't divide by 0");
    dbg!(result2);
}

pub fn get_quotient(dividend : f32, divisor : f32) -> Result<f32,String>{
   if divisor == 0.0 {
       //panic!() throws error and ends program execution so following line of panic!() wont executed
       // panic!("can't divide by 0");
       Err(String::from("Divisor can't be 0"))
   }else {
       Ok(dividend/divisor)
   }
}