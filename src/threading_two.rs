use std::thread;
use std::time::Duration;

pub fn run(){
    let v : Vec<i32>= (1..=5).collect();
    let v1 = v.clone();

    // using move keyword in front of closure we are telling Rust explicitly that
    // move the ownership of all resources used by this child thread to the child thread from main thread
    let handler = thread::spawn(move||{
        for i in v1 {
            println!("child1 thread : {}",i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    dbg!(&v); // valid because we moved ownership of v1(clone of v) not v

    let handler2 = thread::spawn(||{
       for num in v {
           println!("child2 thread : {}",num);
           thread::sleep(Duration::from_secs(1));
       }
    });
    //dbg!(&v);// not valid because we moved ownership of v

    // telling main thread to wait until child thread finishes it's execution
    handler.join().unwrap();
    handler2.join().unwrap();
}
