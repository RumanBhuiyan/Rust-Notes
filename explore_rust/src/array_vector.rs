pub fn run(){
    let mut numbers = vec![];

    for i in 1..=5 {
        numbers.push(i);
    }

    dbg!(&numbers);
    println!("{}",numbers[0]);
    print_vector_items(&numbers);
    dbg!(&numbers);

    let numbers2 = numbers.clone();
    dbg!(numbers2);
}

pub fn print_vector_items(numbers : &Vec<i32>){
    for num in numbers {
        println!("{}",num);
    }
}