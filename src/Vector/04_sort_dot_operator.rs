#![allow(unused)]

/*
    . operator converts v into &mut v implicitly for us.
 */

pub fn run(){
    let mut v = vec![2, 1, 3, 5, 4];
    println!("Before sorting : {:?}", v);

    // sorting : first line is implicitly converted into second line by . operator
    v.sort();
    (&mut v).sort();

    println!("After sorting : {:?}", v);
}