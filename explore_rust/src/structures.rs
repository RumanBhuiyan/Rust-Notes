#![allow(dead_code,unused_variables)]
pub struct Box {
    width : f32,
    height : f32
}

impl Box {
    // self binds method with structure instance and &self borrows instance properties.
    // if self is used instead of &self then instance width,height will be moved to this scope
    // and no longer will be available in other functions scope like get_height(),get_area() etc.
    fn get_width(&self) -> f32{
        return self.width;
    }
    fn set_width(&mut self,w : f32) {
        self.width = w;
    }
    fn get_height(&self) -> f32 {
        return self.height;
    }
    fn set_height (&mut self,h : f32){
        self.height = h;
    }
    fn get_area(&self) -> f32 {
        self.width * self.height
    }
    // static methods
    // declaring static method, as self is missing so static methods aren't bind with structure instance
    fn is_square(width : f32, height : f32) -> bool {
        return width == height
    }
    // we can use Self below in place of Box. Self is preferable than Box because if we change Box name to anything else
    // then we have to change this name in both of the places below.
    fn new_symmetric_box() -> Box {
        Box {
            width : 1.0,
            height : 1.0
        }
    }
    fn new(w : f32, h : f32)-> Self {
        println!("Creating new Box.....");
        Self {
            width : w,
            height : h
        }
    }
}

pub fn run(){
    //let mut rectangle = Box { width : 2.4,height : 3.5}; // use this approach when constructor is absent
    let mut rectangle = Box::new(2.4,3.5); // use this approach when constructor is present
    let square = Box::new_symmetric_box();

    // self refers rectangle object
    rectangle.set_width(3.0);
    dbg!(rectangle.get_width());
    rectangle.set_height(3.0);
    dbg!(rectangle.get_height());
    dbg!(rectangle.get_area());
    dbg!(Box::is_square(rectangle.width,rectangle.height));
    print_structure_details(&square); // passing structure to a function & handling it
}

pub fn print_structure_details(b : &Box) {
   println!("box width : {} box height : {}",b.width,b.height);
}