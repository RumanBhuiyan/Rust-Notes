/*
    collect() is a consumer of an iterator which traverse the whole iterator and collects items
            into the type that you specify or rust can infer.

     N.B => collect() can change that container type which is dynamic like vector, vector deque etc.
            and fails for static type like array as collect detects size in run time not in compile
            time.
 */

pub fn run() {
    // example - 01 (letting Rust infer the type)
    let numbers : Vec<_> = (1..=5).collect();
    println!("{:?}", numbers);

    // example - 02 (Specifying type for Rust)
    let numbers : Vec<i8> = (1..=5).collect();
    println!("{:?}", numbers);

    /* example - 03 : collect() can't change all items type of a container so we have to change
    type of an item by map then collect change the container type to Vector or something else */

    let numbers = (1..=5).map(|x| x as f32).collect::<Vec<f32>>();
    println!("{:?}", numbers);

    // example - 04
    let numbers: Vec<f64> = (1..=5).map(|x| x as f64).collect();
    println!("{:?}", numbers);
}