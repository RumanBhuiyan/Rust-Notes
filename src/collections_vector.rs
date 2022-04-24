/* vector is implemented using stack(LIFO) and it takes memory from heap as its size is determined in run time
 Vector can be created using new() static method of Vec struct or vec![] macro */
pub fn run(){
    // Declaring vector
    let mut odd_numbers = Vec::new();
    let mut even_numbers = vec![];

    // adding item (passing mutable reference to function)
    add_item(&mut odd_numbers,1);
    add_item(&mut even_numbers,2);

    // printing item (passing immutable reference to function)
    print_items(&odd_numbers);
    print_items(&even_numbers);

}

pub fn add_item(v :&mut Vec<i32>,mut initial : i32){
       while initial <= 10 {
           v.push(initial);
           initial += 2;
       }
}

pub fn print_items(v : &Vec<i32>){
    for num in v{
        println!("{}",num);
    }
}