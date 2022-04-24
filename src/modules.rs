#![allow(dead_code)]

// module is used to organize our code nicely.
pub mod helper_functions{

    pub fn print_module_name(){
        println!("Module name is helper_functions")
    }

    pub fn get_sum(a : f64,b : f64) -> f64 {
        return a + b;
    }

    pub fn get_product( a : f32,b : f32) -> f64{
        return a as f64 * b as f64
    }
}