pub fn run() {
    let numbers = vec![1,2,3,4,5,6,7,8];

    let numbers:Vec<_> = numbers.
                            iter()
                            .filter(|x| **x % 2 == 0)
                            .map(|x| x*2)
                            .collect();

    println!("{:?}", numbers);
}