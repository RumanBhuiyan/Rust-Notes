pub fn run(){
    dbg!(get_sum::<i16>(2i16,3i16)); // calling with turbo-fish(::)
    dbg!(get_result(2,3)); // calling without turbo-fish
}
pub fn get_sum<T : std::ops::Add<Output = T>>(number1 : T,number2 : T) ->T{
    number1 + number2
}

pub fn get_result<T:std::cmp::PartialOrd + std::ops::Sub<Output = T>+std::ops::Add<Output = T>>(a : T,b : T) -> T{
    if a>b{ return a -b }
    return a+b
}