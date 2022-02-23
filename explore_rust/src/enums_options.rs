enum Gender {
    Male,
    Female
}
fn print_gender(gender : Gender){
    match gender {
        Gender::Male => println!("Male"),
        Gender::Female => println!("Female")
    }
}
pub fn run(){
    let gender1 = Gender::Male;
    let gender2 = Gender::Female;
    print_gender(gender1);
    print_gender(gender2);

    match get_quotient(3 as f32,2 as f32){
        Some(result) => println!("Quotient : {}",result),
        None => println!("divisor can't be 0")
    }
}
// Rust doesn't support null keyword instead it takes None and None exists in Option<> enum
// if any function can return null/None then we should use Option<returning_value_type> type enum as return item
fn get_quotient(dividend : f32, divisor : f32) -> Option<f32>{
    if divisor == 0.0 {
        return None;
    }
   Some( dividend/divisor)
}