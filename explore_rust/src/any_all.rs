// any returns true if any of the items of iterator pass a condition otherwise false
// all returns true if all of the items of iterator pass a condition otherwise false
pub fn run(){
    let numbers = vec![1,2,3,4,5];

    let result  = numbers.iter().any(|x|x> &4); // true
    let result2 = numbers.iter().all(|x| x> &0); // true

    dbg!(result);
    dbg!(result2);
}