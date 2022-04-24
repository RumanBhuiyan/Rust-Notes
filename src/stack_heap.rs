/* stack allocated values are passed to function as copy so any changes to that variable wont affect
the main variable.you can also pass as &variable_name like heap allocated variables.heap allocated
variables are costly to copy so if you send them as it is to function then ownership moved to function scope */

pub fn run(){
    let mut stack_number = 23;
    let mut heap_number  = Box::new(23);

    stack_var(stack_number);
    dbg!(&stack_number);

    stack_var2(&mut stack_number);
    dbg!(&stack_number);

    heap_var(&mut heap_number);
    dbg!(&heap_number);
}

pub fn stack_var(mut number : i32){
    number += 2;
    println!("stack number in outer function : {}",number)
}

pub fn stack_var2( number : &mut i32){
    *number += 2;
    println!("stack number in outer function : {}",number)
}

pub fn heap_var(number : &mut i32){
    *number +=2;
     println!("Heap number in outer function : {}",number)
}