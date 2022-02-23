// https://doc.rust-lang.org/std/collections/
// vector is implemented using stack and it takes memory from heap as its size is determined in run time
pub fn run(){
    // Vector can be created using new() static method of Vec struct or vec![] macro
    let mut odd_numbers = Vec::new();
    let mut even_numbers = vec![2];

    // adding item
    odd_numbers.push(1);
    even_numbers.push(4);

    dbg!(odd_numbers);
    print_items(&even_numbers);
    println!("Does even numbers contain 2 ?: {}",&even_numbers.contains(&2));
    println!("even numbers vector length : {}",&even_numbers.len());
}
pub fn print_items(v : &Vec<i32>){
    for num in v{
        println!("{}",num);
    }
}