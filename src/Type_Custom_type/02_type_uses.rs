pub fn main(){
    /*
        Later, If i have to change data type of parameters of the function below then i just need to
        change type Number = <data_type> instead of changing data types for 3 times in the
        function signature below.
     */
    fn get_sum(a: Number, b: Number) -> Number{
        a + b
    }

    // calling procedure : - 01
    // type Number = i32;
    // println!("2 + 3 = {}", get_sum(2, 3));

    // calling procedure : - 02
    // type Number = f32;
    // println!("2.1 + 3.3 = {}", get_sum(2.1f32, 3.3f32));

    // calling procedure : - 03
    // N.B => If you don't specify type then Rust compiler will try to convert type
    type Number = f64;
    println!("2.125 + 3.696 = {}", get_sum(2.125 as f64, 3.696 as f64));
}