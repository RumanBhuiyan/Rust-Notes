// Rust offers Result<T,E> type and match statements to handle errors
pub fn run(){
    let age:i8 = 16;

    match is_eligible_for_voter(age) {
        Ok(_) => println!("You are eligible for being voter"),
        Err(response) => println!("You are {}",response)
    }
}

// If Ok() then returning value will be bool otherwise returning value will be String
pub fn is_eligible_for_voter(age : i8) -> Result<bool,String> {
    if age>=18 {
        return Ok(true);
    }
    return Err("Not eligible for being voter".to_string());
}

