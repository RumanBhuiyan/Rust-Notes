use std::io;

pub fn run() {
    let mut first_name = String::new();
    let mut last_name = String::new();

    // Read first name
    match io::stdin().read_line(&mut first_name) {
        Ok(_) => {
            first_name = first_name.trim().parse().unwrap();
        }
        Err(e)=>{
            println!("{}",e);
        }
    }
    // Read second name
    io::stdin().read_line(&mut last_name).unwrap();
    last_name = last_name.trim().parse().unwrap();

    println!("Hello {} {}",first_name,last_name);
}