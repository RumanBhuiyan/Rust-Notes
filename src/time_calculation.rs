// https://www.nofuss.co.za/blog/rust_calculate_execution_time_cli/
use std::time;

pub fn run(){
    // process : 01 (using SystemTime)
    let start = time::SystemTime::now();
    get_sum(1000000);
    let end = time::SystemTime::now();
    println!("Program took {:?}",end.duration_since(start).unwrap());

    // process : 02 ()
    let start = time::Instant::now();
    get_sum(1000000);
    println!("Program took {:?}",start.elapsed());
}

pub fn get_sum(n :i32) -> i128{
    let mut summation:i128 = 0;
    for i in 1..=n {
        summation += i as i128;
    }
    return summation;
}