 /* any() returns true if any of the items of iterator pass a condition otherwise false
 all() returns true if all of the items of iterator pass a condition otherwise false */

pub fn run(){
    let numbers = vec![1,2,3,4,5];

    // any()
    println!("{}",numbers.iter().any(|x| *x>4)); // true
    println!("{}",numbers.iter().any(|&x|  x<0)); // false
    println!("{}",numbers.iter().any(|x| x>&4)); // true

    // all()
    println!("{}",numbers.iter().all(|x| *x>0)); // true
    println!("{}",numbers.iter().all(|&x|  x<0)); // false
    println!("{}",numbers.iter().all(|x| x>&0)); // true
}