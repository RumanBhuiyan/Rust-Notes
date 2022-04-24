//  ownership and borrowing is applicable only for reference type(which are passed by reference)
// not for primitive types like i8, i32 etc because primitive type values are cloned first then
// sent. Rust ownership has few rules like , only one variable can be owner of a value and other
// can borrow the value but can't be owner at the same time.only one variable can mutably borrow.
// for maintaining these rules rust brought the concept of borrowing.

#![allow(unused_assignments)]
pub fn run(){
    let mut my_number = 3;

    make_even(my_number);
    dbg!(my_number); // 3

    make_even2(&mut my_number);
    dbg!(my_number); //4

    let mut numbers : Vec<i32> = vec![];
    add_item(&mut numbers); // mutably borrow numbers vector to add item
    print_item(&numbers); // immutably borrow numbers vector to print its items
}

pub fn make_even(mut num:  i32) {
    if num%2 != 0 {
        num +=1;
    }
}

pub fn make_even2(num: &mut i32) {
    if *num%2 != 0 {
        *num +=1;
    }
}

pub fn add_item(numbers : &mut Vec<i32>) {
    for i in 1..=5{
        numbers.push(i);
    }
}

pub fn print_item(numbers : &Vec<i32>){
    for num in numbers{
        println!("{}",num);
    }
}