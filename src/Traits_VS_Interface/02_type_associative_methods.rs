#![allow(unused)]

use std::io::{stdin};

#[derive(Debug)]
struct Box{
    length : f32,
    width : f32,
}

trait BoxFuncs{
    type BoxDimensionType;
    fn new(length : Self::BoxDimensionType, width : Self::BoxDimensionType) -> Self;
    fn show_dimensions(&self);
}

impl BoxFuncs for Box{
    type BoxDimensionType = f32;

    fn new(length: Self::BoxDimensionType, width: Self::BoxDimensionType) -> Self {
        Self{
            length,
            width
        }
    }

    fn show_dimensions(&self) {
        println!("Length : {} Width : {}", self.length, self.width);
    }
}

pub fn run(){
    let b1 = Box::new(2.1, 3.2);
    b1.show_dimensions();
}
