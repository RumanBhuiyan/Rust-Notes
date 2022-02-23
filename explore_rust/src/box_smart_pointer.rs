//Rust allocates everything on the stack by default for fast memory allocation and accessing value
//You can store things on the heap by wrapping them in smart pointers like Box.
pub fn run(){
    // number is in stack but it holds the memory address of 3 in heap
    let number = Box::new(3);
    dbg!(&number);
    dbg!(*number); // access value by de-referencing operator
}