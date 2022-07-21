use std::env::{args};

/*
    look => arguments.next() is changing the pointer of argument in each next() call so that
            arguments.next() can return you the next element instead of previous element. That's
            why if you dont declare arguments as mutable Rust compiler will tell you to do so.
            Don't be panic! if your declared a variable which isn't changed though you declared it as
            mutable then rust will inform you about that as a warning to fix it.

     N.B => for skipping first argument of command line input we can use like args().skip(1)
 */

pub fn run() {
    // Process - 01 : consuming command line arguments manually using Iterator
    let mut arguments = args();
    let first_arg = arguments.next().unwrap();
    let source = arguments.next().unwrap();
    let destination = arguments.next().unwrap();

    println!("first_arg : {}", first_arg);
    println!("source : {}", source);
    println!("destination : {}", destination);

    // Process - 02: consuming command line arguments using for loop
    let mut arguments = args();
    for arg in arguments{
        println!("{}",arg);
    }
}
