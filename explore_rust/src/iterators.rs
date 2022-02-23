// if any data type has iterator trait implemented then we can iterate over its item by .next()
// in each next() of iterator returns a memory address/reference by which we can access the value or change value
pub fn run(){
    let mut numbers = vec![1,2,3,4,5,6];

    // process : 01
    let mut iterator = numbers.iter();
    // dbg!() is nothing but pretty print of {:?}
    //as iterator.next() returns Option<> type so value comes as Some() to extract value only use unwrap()
    println!("{:?}",iterator.next());
    println!("{:?}",iterator.next().unwrap());

    // immutable borrow : Process 01 (&numbers returns vector reference/ memory address)
    for number in &numbers {
        println!("immutable borrow {}",*number); // access memory address value by * which is de-referencing operator
        // println!("immutable borrow {}",number); // does the same job
    }
    // immutable borrow : Process 02 (numbers.iter() returns vector reference itself)
    for number in numbers.iter() {
        println!("{}",number);
    }

    // mutable borrow : process 01
    for number in &mut numbers {
        *number *= 3;
    }
    // mutable borrow : process 02
    for number in numbers.iter_mut(){
        *number *= 2;
    }
    dbg!(numbers);
}