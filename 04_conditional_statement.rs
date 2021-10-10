fn main(){
	let number = get_input();

	if number > 0 {
		println!("Positive Number");
	}else if number < 0 {
		println!("Negative Number");
	}else {
		println!("Zero");
	}
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}