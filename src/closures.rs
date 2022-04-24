// closures are anonymous functions in rust
pub fn run(){
    // closure : |parameter1,parameter2| {// business logic}; there is no need of {} for 1 statement

    // type - 01 : single parameter with explicit type annotation
    // N.B => for single statement there is no need of curly braces and mentioning return type.
    // let Rust do that
    let square = |x : f32| x * x ;
    println!("Square of 2 : {}",square(2 as f32));

    // type - 02 :  let Rust detect type , If rust detects type then it won't allow other types
    let square2 = |x| x * x;
    println!("Square of 2 : {}",square2(2));
    //println!("Square of 2 : {}",square2(2.5)); // expected integer but found floating point number

    // type - 03 : embracing multiple statements inside {}
    let sum = |x, y| {
        let summation = x + y;
        summation
    };
    println!("Summation of 2 + 3 = {}",sum(2, 3));

    // type - 04 : multiple parameter with explicit type annotation and multiple statement in functions body
    let factorial = |mut x: i32| -> i32 {
        let mut product = 1;
        while x >1 {
            product *= x;
            x -= 1;
        }
        product
    };
    println!("Factorial of 5 : {}",factorial(5));
}