/*
    unwrap() : For Result<T,E> or Some<T> Type returns the unpacked value from Ok(), Some() variants
                and panic!() for Err() or None variants otherwise.

    ? macro  :  For Some<T> Type it returns unpacked values Some() variants or None otherwise

    Look => That means, Here ? can return plain data types value from Some() or None otherwise.
            So is there any data type in Rust that we can use as returning statement for consumer
            function ? Yes. That is Option<T> type.
 */
pub fn run() {
    println!("{:?}", consumer(5 as f32, 2 as f32));
    println!("{:?}", consumer(5 as f32, 0 as f32));
}

pub fn consumer(dividend : f32, divisor : f32) -> Option<f32>{
    Some(get_quotient(dividend, divisor)?)
}

pub fn get_quotient(dividend : f32 , divisor: f32)-> Option<f32>{
    if divisor != 0.0{
        Some(dividend / divisor)
    }else{
        None
    }
}