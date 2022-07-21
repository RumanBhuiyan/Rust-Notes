pub fn main(){
    // look stack allocated a isn't dropped
     let a = 5;
     stack(a);
     println!("main scoped a = {}",a);

    // look heap allocated b is dropped
    let b = Box::new(5);
    heap(b);
    // println!("b = {}",b);
}

pub fn stack(num : i32){
    println!("local scoped copied a= {}", num);
}

pub fn heap(num : Box<i32>){
    println!("local scoped b= {}", num);
}