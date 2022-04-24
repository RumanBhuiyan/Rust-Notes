// heap variables have to be borrowed into function otherwise rust will drop them after going out of scope
pub fn run(){
    let number = 24; // number = 24 in stack
    print_from_stack(number); // 24
    print_from_stack(number); // 24

    let number2 = Box::new(32); // number2 = 32 in heap
    print_from_heap2(&number2); // borrow number2
    print_from_heap2(&number2); // borrow number2

    let name = String::from("Ruman");
    print_from_heap(&name);
}
pub fn print_from_stack(stack_var : i32){
    println!("{}",stack_var);
}
pub fn print_from_heap(heap_var : &String) {
    println!("{}",heap_var);
}
pub fn print_from_heap2(heap_var : &Box<i32>){
    println!("{}",heap_var);
}
