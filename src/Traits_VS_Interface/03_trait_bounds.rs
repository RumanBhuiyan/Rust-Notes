#![allow(unused)]

trait HasEven{
    fn is_even(self) -> bool;
}

impl HasEven for f32 {
    fn is_even(self) -> bool {
        println!("is_even() for f32 is called !");
        self % 2f32 == 0f32
    }
}

impl HasEven for i32 {
    fn is_even(self) -> bool {
        println!("is_even() for i32 is called !");
        self %2 == 0
    }
}

/*
    where Number : HasEven ; this is called trait bound. When we try to write generic function
        then we take parameter types dynamically but for which type which function should be called
        by Rust, you will have to tell explicitly. You can do so by using a trait having the
        methods which you wanna have for your types. Finally implement trait methods.
 */
fn check_even<Number> (num : Number) -> bool where Number : HasEven {
    num.is_even()
}

pub fn main(){
    let num1 = 2i32;
    let num2 = 2.5f32;

    println!("Is {} even ? {}", num1, check_even(num1));
    println!("Is {} even ? {}", num2, check_even(num2));
}