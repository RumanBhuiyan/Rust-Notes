fn main(){

	// while loop
	let mut counter = get_input();
	while counter > 0{
		println!("Hello world");
		counter -=1;
	}
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}