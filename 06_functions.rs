fn main(){

    let first = get_input();
    let second = get_input();
 
    println!("Summation {}",get_sum(first,second));
    println!("Product is {}",get_product(first,second));
 }
 
 fn get_sum(a:i32,b:i32) -> i32{
     return a+b;
 }
 
 fn get_product(a:i32,b:i32) -> i32{
     return a*b;
 }
 
 fn get_input() -> i32{
 
     let mut line  = String::new();
     std::io::stdin().read_line(&mut line).unwrap();
     let number : i32 = line.trim().parse().unwrap();
     return number ;
 }