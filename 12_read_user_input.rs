fn main() {

    let first_number = get_input();
    let second_number = get_input();

    println!("Summation : {}",first_number+second_number);

}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}