#![allow(unused)]

/*
    Scope starts at the time of declaration and lifetime start with initialization.
    So scope and lifetime aren't similar

    Dropping values follows the reverse order of declaration or scope. At the time of dropping
    variable of any scope Rust also drops resources hold by that variable. For the example below,
    declaration sequence : a -> b -> c
    dropping sequence    : c -> b -> a
 */

struct X(char);

impl Drop for X{
    fn drop(&mut self) {
        println!("Dropping {}",self.0);
    }
}

pub fn main(){
    let a = X('a');
    let b;
    let c = X('c');
    b = X('b');
}