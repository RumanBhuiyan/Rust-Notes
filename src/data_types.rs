// Why memory space becomes power of 2^n like 8,16,32,64,128 ?
// machine generates same instructions for  19 bits and 32 bits so there is no benefit of selecting 19 bits over 32 bits.
// if we allocate 19 bits then machine will add extra bits to be 32 bits which is called padding.this padding
// makes program in-efficient. If there are many different types then there must be a lot of machine code even if
// source code is same. This is called code bloat. As instructions for data types come from CPU cache so limited
// data types can be fitted into CPU cache perfectly.
//
// Generally isize , usize are used to contain memory address
//     usize takes memory basis on system like if computer is 32 bit then usize->u32 ,otherwise usize->u64
// isize takes memory basis on system like if computer is 32 bit then isize->i32 ,otherwise isize->u64

#![allow(unused_variables)]
pub fn run(){
    // various data types
    let a : i8 = 8;
    let b : i16 = 16;
    let c : i32 = 32;
    let d : i64 = 64;
    let answer :bool = true;
    let alphabet : char = 'R';

    println!("binary : {:b}",a);
    println!("Octal : {:o}",a);
    println!("hexadecimal : {:x}",a);

    let numbers =[1,23,4,5];
    let index1 : usize = 2;

    let index2 : isize = 3;
    dbg!(index1);
    dbg!(index2);
}