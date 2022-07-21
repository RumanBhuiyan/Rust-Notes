#![allow(unused)]

/*
    To avoid collision with mutable and immutable borrowing rules, perform your mutable borrow
        operations in a separate block so that mutable borrower is gone out of scope after mutating value.
        So that we can create another mutable borrower to perform another task. There is no sufferings
        for immutable borrowing as you can take any number of immutable borrowers as you want.
 */

pub fn run(){
    let mut a = 12;
    println!("a : {}",a);

    {
        let b = &mut a;
        *b += 2;
        println!("a : {}",a);
    }

    {
        let c = &mut a;
        *c += 2;
        println!("a : {}",a);
    }
}