pub fn run() {
    let mut numbers = vec![1,2,3,4,5];

    // Immutable iteration
    // Process - 01
    for num in &numbers{
        println!("{}", *num);
    }

    // Process - 02
    for num in numbers.iter(){
        // both of the statement below works same
        // println!("{}", num);
        println!("{}", *num);
    }

    // Mutable iteration
    // Process - 01
    for num in &mut numbers{
        *num *= 2;
    }
    println!("After multiplying by 2 : {:?}", &numbers);

    // Process - 02
    for num in numbers.iter_mut(){
        *num *= 3;
    }
    println!("After multiplying by 3 : {:?}", &numbers);
}