pub fn run(){
    let numbers = vec![-4,-3,-2,-1,0,1,2,3,4,5,6];

    let positive_evens : Vec<i32> = numbers.iter()
                                .filter(|x| **x > 0 && **x%2 == 0)
                                .map(|x| *x)
                                .collect();
    dbg!(positive_evens);
}