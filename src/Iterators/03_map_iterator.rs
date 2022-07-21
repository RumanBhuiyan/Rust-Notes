/* map takes an iterator and returns a new iterator with the strategy mentioned in lambda
     function on how to change each item in each iteration

   N.B => As map() takes an iterator and numbers itself isn't an iterator that's why we have to
            pass an iterator of numbers like numbers.iter() in example 02 unlike example -01
*/

pub fn run() {
    // example - 01, Here using Vec<_> let Rust infer the type for you
    println!("{:?}", (1..=5).map(|x| x*2).collect::<Vec<_>>());

    // example - 02
    let numbers = vec![1,2,3,4,5];
    let numbers = numbers.iter().map(|x| x *2).collect::<Vec<_>>();
    dbg!(&numbers);
}