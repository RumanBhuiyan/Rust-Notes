use std::env::args;
use std::process::exit;

pub fn run(){
    let args : Vec<String> = args().collect();

    if args[1].parse::<i32>().unwrap() <= 0{
        exit(76);
    }else{
        println!("{:?}",args);
    }
}