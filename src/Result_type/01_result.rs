pub fn main(){
      // Process - 01 : consuming Result types variants value using match expression
    let response = get_quotient(2.5, 3.);
    match response{
        Ok(result) => println!("result : {}", result),
        Err(message) => println!("message : {}", message)
    }

    // Process - 02 : consuming Result types Ok() variant outputs value by unwrap()
    // unwrap() returns the Ok() variants value otherwise panics! if Err()
    println!("Result : {:?}", get_quotient(5.2, 2.).unwrap());

    // Process - 03 : consuming Result types value by is_ok(), is_err()
    println!("is_ok() ?: {:?}", get_quotient(5.2, 2.).is_ok());
    println!("ok() ?: {:?}", get_quotient(5.2, 2.).ok().unwrap());

    println!("is_err() ?: {:?}", get_quotient(5.2, 0.).is_err());
    println!("err() ?: {:?}", get_quotient(5.2, 0.).err().unwrap());
}

// Result<Ok_variant_type, Err_variant_type>
pub fn get_quotient(dividend: f32, divisor: f32) -> Result<f32, &'static str> {
    if divisor == 0 as f32 {
        Err("Divisor can't be zero")
    }else{
       Ok( dividend / divisor)
    }
}