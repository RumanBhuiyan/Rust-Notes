#![allow(unused)]

pub fn main(){
    let mut numbers = vec![1,2,3,4,5];

    // Process - 01 : mutable borrow
    for num in &mut numbers{
        *num *= 2;
    }
    dbg!(&numbers);

    // Process - 02: mutable borrow inside a function
    make_double(&mut numbers);
    dbg!(&numbers);
}

pub fn make_double(nums : &mut Vec<i32>) {
    for num in nums.iter_mut(){
        *num *= 2;
    }
}
