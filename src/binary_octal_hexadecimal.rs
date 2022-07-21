fn run(){
    // Declaring binary, octal, hexadecimal numbers
    let decimal = 10;
    let _binary = 0b101;
    let _octal = 0o11;
    let _hexadecimal = 0xff;

    // representing binary, octal, hexadecimal numbers
    println!("decimal : {}",decimal);
    println!("binary : {:b}",decimal);
    println!("octal : {:o}",decimal);
    println!("hexadecimal : {:x}",decimal);

    // exponential numbers have 2 part.
    // part before e is called mantissa and part after e is exponent.
    // in gravity mantissa = 981 and exponent = -2
    let gravity = 981e-2;
    println!("gravity : {}",gravity);
}