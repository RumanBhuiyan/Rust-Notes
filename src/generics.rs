// The process using which rust converts generic code into concrete code is called monomorphization
#![allow(dead_code,unused_assignments,unused_variables)]
use std::fmt::Display;

trait Printable {
    fn print_book_id(&self);
}
struct Book<T>{
    book_id : T
}
// in general : impl interface_name for struct_name, for generics following style is used
// here Display is Trait
impl <T:Display>  Book<T>  {
    fn print_id(&self){
        println!("book-id : {}",self.book_id);
    }
    fn get_book_id(&self) -> &T {
        &self.book_id
    }
}
pub fn run(){
    // primitive type generics value can be displayed by dint of Display trait
    print_value(2 as i32);
    print_value(String::from("32"));

    // custom type generic
    let b1 = Book{book_id:2};
    let b2 = Book{book_id:String::from("32")};

    b1.print_id();
    b2.print_id();
    dbg!(b1.get_book_id());
}
// generic function
pub fn print_value<T:Display>(t : T){
    println!("{}",t);
}