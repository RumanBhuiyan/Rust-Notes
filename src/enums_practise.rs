#![allow(unused)]
// enum is mainly used for defining our custom data type
//Scenario -01: enum without data binding
#[derive(Debug)]
enum IpAddress {
    V4,
    V6
}

// Scenario - 02 : enum with data binding
#[derive(Debug)]
enum MyTypes{
    Point{ x : f32, y : f32}, // declaring my custom struct type
    Color(u8, u8, u8), // declaring tuple struct
    LaptopType(IpAddress),
}
// implementing methods for MyTypes instances
impl MyTypes{
    fn get_point(&self) -> (f32, f32){
        match &self {
            /* As &self matches with MyTypes::Point{} that's why value inside Point{}
             is bound with x, y. That's how we can get the value out of Custom data
            type variant or Option variant */
            MyTypes::Point { x, y } => (*x, *y),
            _ => (0.0, 0.0)
        }
    }
    fn get_color(&self) -> (u8, u8, u8){
       match &self{
           Self::Color(r, g, b) => (*r, *g, *b),
           _ => (255, 255, 255)
       }
    }
    fn which_version(&self){
        match &self{
            Self::LaptopType(version)=> if  let IpAddress::V4 = *version{
                println!("You are using IPV4")
            }else{
                println!("You are using IPV6")
            },
            _ => println!("version unknown")
        }
    }
}

pub fn run(){
    let my_laptop = IpAddress::V4;

    // I am creating instance of my own custom types defined in MyTypes enum
    let center = MyTypes::Point {x : 3.5, y : 2.5};
    let green = MyTypes::Color(1,2,3);
    let laptop = MyTypes::LaptopType(IpAddress::V4);

    // dbg!(&center);
    // dbg!(&green);
    // dbg!(&laptop);

    let center = center.get_point();
    println!("Center x : {} y : {}",center.0, center.1);

    let green = green.get_color();
    println!("green red : {} green : {} blue : {}",green.0,green.1,green.2);

    laptop.which_version();
}