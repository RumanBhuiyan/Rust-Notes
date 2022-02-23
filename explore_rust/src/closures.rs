// closures are anonymous functions in rust
pub fn run(){
    // closure : |parameter1,parameter2| {// business logic}; there is no need of {} for 1 statement
    // example : 01
    let is_even  = |x| x%2 ==0;
    // example : 02
    let is_positive = |x| {
        if x>0{
            println!("Positive");
        }else {
            println!("Negative");
        }
    };
    // example : 03
    let get_product = |a: i32,b: i32| ->i32 { a * b };

    println!("Is 2 even : {}",is_even(2));
    is_positive(3);
    println!("2*3 = {}",get_product(2,3));
}