/* when you need struct type pattern but you don't need named fields then pick tuple-struct
this helps to print tuple-struct instance with {:?} formatter */

#[derive(Debug)]
struct Person(
    String,
    i8,
);

pub fn run(){
    let p1 = Person(String::from("Max"),25);
    dbg!(p1);
}