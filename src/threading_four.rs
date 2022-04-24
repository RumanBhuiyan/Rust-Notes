use std::sync::Mutex;

pub fn run(){
    // Mutex = Mutual exclusion; creating a resource which can be locked before accessing and
    // that lock can be released after accessing
    let number = Mutex::new(5);
    {
        // locking the resource before accessing and the lock will be released automatically
        // when execution goes out of scope
        let mut number2 = number.lock().unwrap();
        *number2 = 6;
        println!("Now number is {}",number2)
    }
    println!("Now number is {:?}",number.lock().unwrap());
}