#![allow(unused)]

#[derive(Debug)]
struct Box{
    length: f32,
    width : f32,
    height : f32,
}
/*
    spreading syntax is also available in Rust. Look, we are taking reference of Box instances
    so at first we need to de-reference it to get its all value, only height attributes value
    will copied by ..*b into same_height() method.
 */
impl Box {
    fn new(length : f32, width : f32, height : f32) -> Self{
        Self{
            length,
            width,
            height
        }
    }
    fn same_height(length : f32, width : f32, b : &Box) -> Self{
        Self{
            length,
            width,
            ..*b
        }
    }
}

pub fn main() {
    let b1 = Box::new(2.5, 3.2, 1f32);
    let b2 = Box::same_height(2.8, 3.5, &b1);

    println!("b2 : {:?}", b2);
    println!("b1 : {:?}", b1);
}