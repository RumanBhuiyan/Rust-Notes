pub fn run(){
    // process : 01
    // "4" is converted to String as mentioned in left
    let a : String = "4".parse().unwrap();
    println!("a : {}",a);

    // process : 02
    // now a is converted to i32 as mentioned. unwrap() returns only Ok() value
    let b = a.parse::<i32>().unwrap();
    println!("b : {}",b);

    // process : 03 (using as operator. N.B: as is used to type cast within primitive types)
    let c = b as i64;
    println!("c : {}",c);
    println!("Summation  : {}", a.parse::<i64>().unwrap()+ b as i64+c )
}
