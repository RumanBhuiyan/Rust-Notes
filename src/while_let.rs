pub fn run(){
    let mut numbers = vec![1,2,3,4,5];

    while let Some(num) = numbers.pop(){
        println!("{}",num);
    }
}