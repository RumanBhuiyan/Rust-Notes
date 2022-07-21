/* filter takes an iterator and returns another iterator on top of previous iterator
     so the resulting iterator will be an iterator of iterator. This type of iterator is
     called iterator adapter
*/

pub fn run() {
    // example - 01, Here using Vec<_> let Rust infer the type for you
    println!("{:?}", (1..=5).filter(|x| x % 2 == 0).collect::<Vec<_>>());

    // example - 02
    let numbers = vec![1,2,3,4,5];
    let numbers = numbers.iter().filter(|x| {
            println!("x = {:?}, *x = {:?} **x = {:?}",x, *x, **x); // Here x == *x == **x
            **x % 2 == 0})
        .collect::<Vec<_>>();
    dbg!(&numbers);
}