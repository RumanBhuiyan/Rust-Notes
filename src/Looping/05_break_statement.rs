#![allow(unused)]

/*
    variations of break statement uses:

    break => simply come out of this loop
    break 'level_name => execution is jumped after that level
    break 'level_name expression => jump after that level after returning the expression
 */

pub fn run(){
    // Uses - 01
    for i in 1..5{
        if i % 2 == 0{
            break
        }
        println!("i : {}", i);
    }


    // Uses - 02 : lifetime level
    'last:
    loop {
        for i in 1..10{
            if i%2 == 0{
                break 'last
            }
        }
    }
    println!("Came out of 'last lifetime level");


    // Uses - 03 : loop level as lifetime.
    // N.B => lifetime level should be used with nested loop
    let even_exists = 'outer: loop {
        for i in 1..5 {
            if i %2 == 0 {
                break 'outer true
            }else if i >=4 {
                break 'outer false
            }
            // here else block is omitted which means that else {} is replaced by Rust compiler
        }
    };
    println!("even_exists ? {:?}", even_exists);
}
