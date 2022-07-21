#![allow(unused)]

/*
    Once we mutably borrow the owner then there is a possibility that mutable borrower will
    add new element into it which may require re-allocation like vector. If current items are
    moved into another heap location to fit all items with recently added items then owner and mutable
    borrower both will point to the new heap buffer pointer except immutable borrowers. So now if you
    want to access values by immutable borrower it may be dangling pointer as items may not be there
    due to re-allocation to fit new item. That's why Rust doesn't allow accessing resources by
    immutable borrower if owner is borrowed mutably once. To fix this issue, just immutably borrow
    the owner again to get the new pointer.
 */

pub fn run(){
    let mut numbers = vec![1,2,3,4,5];
    println!("numbers : {:?}", numbers);

    // immutable borrow
    let  numbers2 = &numbers;
    println!("numbers2 : {:?}", numbers2);

    // mutable borrow
    let mut numbers3 = &mut numbers;
    numbers3.push(100);
    println!("numbers3 : {:?}", numbers3);
    println!("numbers : {:?}", numbers);

    let  numbers2 = &numbers;
    println!("numbers2 : {:?}", numbers2);
}