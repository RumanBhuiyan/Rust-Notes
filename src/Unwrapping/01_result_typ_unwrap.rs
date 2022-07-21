/*
    unwrap() returns unpacked value from Ok() variant and panics!() with the
        error message from Err() variant for Err() variant.
*/
pub fn main() {
    println!("{:?}", a(true));
    println!("{:?}", a(true).unwrap());
    println!("{:?}", a(false));
    println!("{:?}", a(false).unwrap());
}

pub fn a(flag : bool) ->Result<i32, String>{
    if flag{
        Ok(47)
    }else{
        Err("Error".to_string())
    }
}