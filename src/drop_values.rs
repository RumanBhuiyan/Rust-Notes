#![allow(unused, dead_code)]
// use std::mem::drop;

#[derive(Debug)]
struct Person(String);

/* when any instance of Person will go out scope then drop() method of Drop trait of that instance is called
 if exists otherwise built-in implementation is called */
impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping the value : {:?}",&self.0);
    }
}

/*
    run() performs it tasks using stack so at end stack will look like
    println!()
    p2
    p1
    that's why p2 is dropped before p1.

    N.B => we can't call p1.drop() to manually drop a value though we have drop() implementation of Drop trait.
    but we can use drop(p1) method of std::mem::drop. We don't need to import this using 'use std::mem::drop'
    as this comes with 'prelude'. prelude means default importing.
 */
pub fn run(){
    let p1 = Person(String::from("Roman"));

    // manually dropping value
    // p1.drop(); // Rust doesn't allow it
    //drop(p1); // this can be used just because of use std::mem::drop(), drop(p1) searches drop() implementation
    // of p1 then Rust call it. you can't manually call p1.drop().let rust does that.

    let p2 = Person(String::from("Alex"));
    println!("p1 , p2 created...");
}
