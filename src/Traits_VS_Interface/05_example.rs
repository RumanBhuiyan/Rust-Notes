/* traits in Rust are similar as interface in other OOP languages. traits are implemented for struct
as class is absent in Rust
as like as interface, trait can have both concrete method and abstract method
concrete method -> function with body, abstract method -> function without body
abstract methods must be implemented but implementation of concrete method is optional
implementation of concrete method overrides the implementation inside trait
&self binds this method with struct Color instance so that it can get access of it */

trait ColorMethods {
    fn show_red_value(&self) -> u8;
    fn show_all_values(&self);
    fn show_trait_name(&self){
        println!("Trait name : ColorMethods");
    }
}

struct  Color {
    red: u8,
    green : u8,
    blue : u8
}

impl ColorMethods for Color {
    fn show_red_value(&self) ->u8 {
        self.red
    }

    fn show_all_values(&self) {
       println!("Red : {} Green : {} Blue : {}",self.red,self.green,self.blue);
    }

    // overriding show_trait_name() method and its optional
    fn show_trait_name(&self) {
        println!("Trait name : anything else");
    }
}

pub fn run(){
    let my_color = Color { red : 127,green : 128, blue : 129};

    println!("Red : {}",my_color.show_red_value());
    my_color.show_all_values();
    my_color.show_trait_name();
}