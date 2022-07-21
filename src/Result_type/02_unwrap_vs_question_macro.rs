/*
    unwrap() : For Result<T,E> or Some<T> Type returns the unpacked value from Ok(), Some() variants
                and panic!() for Err() or None variants otherwise.

    ? macro  :  For Result<T,E> it returns unpacked values from Ok() variants
                and for Err() variant it returns unpacked message from Err() variant

    Look => That means Here, ? can return plain data types value or Err() from consumer function.
            So is there any data type in Rust that can satisfy this situation ?
            Yes. You will have take Result<T,E> type into consideration as a return Type for
            consumer function.
 */
pub fn run() {
    println!("{:?}", consumer(5 as f32, 2 as f32));
    println!("{:?}", consumer(5 as f32, 0 as f32));
}

pub fn consumer(dividend : f32, divisor : f32) -> Result<f32, String>{
    Ok(get_quotient(dividend, divisor)?)
}

pub fn get_quotient(dividend : f32 , divisor: f32)-> Result<f32, String>{
    if divisor != 0.0{
        Ok(dividend / divisor)
    }else{
        Err("Can't divide by 0".to_string())
    }
}