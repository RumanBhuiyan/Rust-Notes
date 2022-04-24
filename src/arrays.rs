pub fn run(){
    // array declaration requires [data_type;size] = [initial_value;how_many_times]
    let mut numbers  : [i32;5] = [0;5];
    add_item(&mut numbers); // mutably borrow numbers
    print_item(&numbers); // immutable borrow numbers
}

pub fn add_item(numbers : &mut [i32;5]){
   for i in 0..5 {
       numbers[i] = (i + 1) as i32;
   }
}

pub fn print_item(numbers : &[i32;5]) {
    for num in numbers.iter() {
        println!("{}",num);
    }
}

