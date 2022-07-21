use std::mem::swap;

pub fn run() {
    let mut a = 4;
    let mut b = 5;
    println!("Before swapping  a : {} b : {}", a, b);

    swap(&mut a, &mut b);
    println!("After swapping   a : {} b : {}", a, b);
}