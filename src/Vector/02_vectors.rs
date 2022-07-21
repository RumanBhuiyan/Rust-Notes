/*
    In line no 14, v1 is a stack frame. You can think it like an object of 3 properties.
        i) pointer of data buffer in heap
        ii) length of the vector right now
        iii) capacity of the vector right now
    That's why when you ask for len() of any collection type it takes O(1) time as length is
    in stack frame and Rust compiler won't have to compute it with any traversing.

    Fat pointer : which has 2 properties. i) pointer to data buffer in heap ii) length
 */

#![allow(unused)]
#[derive(Debug)]
enum VecTypes {
    Name(String),
    Age(i32)
}

pub fn run(){
    // way of declaring vector
    let mut v1 = Vec::new();
    let mut v2 = vec![];

    // pushing element into vector
    v1.push(23);
    v1.push(12);

    v2.push(11);
    v2.push(22);

    /* accessing values
    v1[index] panics when index is out of range */
    println!("Second element of v1 : {}",v1[1]);

    // v1.get(index) returns Some() for valid data and None for at end in data
    for i in 0..100{
        match v1.get(i){
            Some(value) => println!("{}",*value),
            None => break
        }
    }

    // iterating over vector items by  iterator
    for item in &v1{
        /* both of the statements below work fine because Rust automatically dereference
         value when we pass item , or we can manually dereference value by *(de-referencing) operator
         & -> referencing operator and * -> de-referencing operator */
        println!("{}",*item);
        // println!("{}",item);
    }

    // lets look into a problem

    /* here i am storing the reference of &v1[1] into keep but if i push new element into v1
     then vector will reallocate memory to fit all items with new item so then reference of keep
     will point to a data which doesn't exist in that location. That's why Rust throws error
     that's how Rust ensure memory safety */
    // let keep = &v1[1];
    // v1.push(58);
    // println!("{}",keep);

    /* this type of practise helps us to avoid above problem
        I am storing second block value of v1 and i don't care whatever v1 references to
     */
    let keep = v1[1];
    v1.push(58);
    println!("{}",keep);

    /*
    vector can contain data of multiple types but we have to tell vector multiple types within
    one type using enum like below.
     */
     let mut v = vec![
        VecTypes::Name(String::from("Ruman")),
        VecTypes::Age(25),
    ];

    dbg!(&v);
}