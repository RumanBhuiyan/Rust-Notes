/*
    unwrap() returns unpacked value from Some() variant
            and panics!() for the None variant
*/
pub fn main() {

    println!("{:?}", a(true));
    println!("{:?}", a(true).unwrap());
    println!("{:?}", a(false));
    println!("{:?}", a(false).unwrap());
}

pub fn a(flag : bool) ->Option<i32>{
    if flag{
        Some(47)
    }else{
        None
    }
}