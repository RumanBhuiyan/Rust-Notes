#![allow(unused)]

// Tuple struct
struct Shape(
    f32,
    f32,
    f32,
);

// Process - 01 : implementing methods for tuple struct directly
impl Shape{
    fn show_dimensions(&self){
        println!("Length : {} Width : {} height : {}", self.0, self.1, self.2);
    }
}

// Process - 02 : implementing methods for tuple struct through trait
trait ShapeFuncs{
    fn get_length(&self) -> f32;
}

impl ShapeFuncs for Shape{
    fn get_length(&self) -> f32 {
        self.0
    }
}

pub fn main(){
    let rectangle = Shape(2f32, 3f32, 4f32);

    rectangle.show_dimensions();
    println!("Length : {}", rectangle.get_length());
}
