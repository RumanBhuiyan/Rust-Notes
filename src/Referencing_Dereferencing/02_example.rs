/*
    As Rust is memory safe and it guarantees memory safety so it won't allow you to access memory
    and alter it. That's why,

    b = &5 ; b holds the memory address where 5 is kept.

    &b ; here compiler tries to access memory address hold by b. As it's not allowed that's why
         compiler provides the ultimate value after auto de-referencing by itself.

    *b ; compiler retrieves the value of memory address b

    *&b ; firstly, compiler reads the memory address of &b then retrieves it's value

    &*b ; compiler firstly retrieves value of *b then tries to provide the address of that value
            as it's not allowed so compiler simply returns the value

 */
#![allow(unused)]
pub fn run(){
    let b = &5;

    println!("b = {}", b); // 5
    println!("&b = {}", &b); // 5
    println!("*b = {}", *b); // 5
    println!("*&b = {}", *&b); // 5
    println!("&*b = {}", &*b); // 5
}