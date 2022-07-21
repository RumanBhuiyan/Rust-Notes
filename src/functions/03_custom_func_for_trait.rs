pub trait CheckEven{
    fn is_even(self) -> bool;
}

impl CheckEven for i32  {
    fn is_even(self) -> bool {
        self %2 == 0
    }
}

pub fn run() {
    let number = 24;
    println!("Is even ?: {}", number.is_even());
    println!("31 ?: {}", 31.is_even());
}