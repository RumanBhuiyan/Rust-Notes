/*
    function which is under a trait that is called method otherwise it's function
    functions or methods can be invoked by 2 ways.
        i) Object oriented style
        ii) functional syntax
     Object oriented style is more readable and less verbose but we should also know about
     functional syntax to understand how those methods exist in different scopes under
     trait implementation.
 */

pub fn run() {
    let mut numbers = vec![1,2,3,4];

    // Process - 01 (invoke of len(), push() in object oriented style)
    println!("Length of numbers : {}", numbers.len());
    numbers.push(5);
    println!("After pushing 5 numbers : {:?}", numbers);

    // Process - 02 (invoke of len(), push() in functional style)
    println!("Length of numbers : {}",<[i32]>::len(&numbers));
    Vec::push(&mut numbers, 100);
    println!("After pushing 100 numbers : {:?}", numbers);
}