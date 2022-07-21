#![allow(unused)]

/*
    Self denotes struct, here Self = Box
    self denotes struct instance, here self = b1
 */

#[derive(Debug)]
struct Box{
    length : f32,
    width : f32,
}

impl Box{
    fn new(length : f32, width : f32) -> Self{
        Self{
            length,
            width
        }
    }
    fn show_dimensions(&self){
        println!("Length : {} Width : {}", self.length, self.width)
    }
}

pub fn run(){
    let b1 = Box::new(2.1, 3.2);
    b1.show_dimensions();
}