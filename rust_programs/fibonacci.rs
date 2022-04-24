pub fn main(){
    println!("Enter the nth fibonacci at max you want : ");
    let limit = get_single_input();

    let mut fibonacci_numbers = vec![0, 1, 1];
    generate_fibonacci_series(limit, &mut fibonacci_numbers);
    // dbg!(&fibonacci_numbers);

    for i in 1..=limit{
        println!("{}th fibonacci = recursive_fibo() : {} \
                                   iterative_fibo() : {} \
                                   fibonacci series : {}
                                   ",i, recursive_fibo(i),iterative_fibo(i), fibonacci_numbers[i as usize]);
        assert_eq!(recursive_fibo(i), iterative_fibo(i), "values don't match for nth = {}",i);
    }
}
// Recursive Solution
pub fn recursive_fibo(n : i32) -> i32 {
    if n == 1 || n == 2 {
        return 1
    }
    return recursive_fibo(n - 1) + recursive_fibo(n - 2)
}
// Iterative Solution
pub fn iterative_fibo(n : i32) -> i32{
    if n == 1 || n == 2{
        return 1
    }

    let (mut fibo, mut n1, mut n2 )= (0, 1, 1);
    for _ in 3..=n {
        fibo = n1 + n2;
        n1 = n2;
        n2 = fibo;
    }
    return fibo;
}

// Dynamic programming Solution
pub fn generate_fibonacci_series(term : i32, series: &mut Vec<i32>){
    let (mut _fibo , mut n1, mut n2) = (0, 1, 1);
    for _ in 3..= term {
        _fibo = n1 + n2;
        series.push(_fibo);
        n1 = n2;
        n2 = _fibo;
    }
}
// Defining a function to utilize again and again
pub fn get_single_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}