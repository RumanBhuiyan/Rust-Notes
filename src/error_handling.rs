// Rust offers Result<T,E> type and match statements to handle recoverable errors
pub fn run(){
    let age:i8 = 16;

    match is_eligible_for_voter(age) {
        Ok(_) => println!("You are eligible for being voter"),
        Err(response) => println!("You are {}",response)
    }
}

// return bool for Ok() variant otherwise String
pub fn is_eligible_for_voter(age : i8) -> Result<bool,String> {
    if age>=18 {
        return Ok(true);
    }
    return Err("Not eligible for being voter".to_string());
}

