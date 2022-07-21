
pub fn run() {
    // Process - 01 : consuming iterators value manually
    let mut numbers = (1..=5);
    loop{
        match numbers.next(){
            Some(value) => println!("{}",value),
            None => break
        }
    }

    // Process - 02 : consuming iterators values using loop
    let mut numbers = (5..=10);
    for num in numbers{
        println!("{}", num);
    }
}