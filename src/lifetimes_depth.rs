#![allow(unused, dead_code)]
pub fn run(){
    let c = get_country_name();
    dbg!(&c);

    let s = get_single_value(&4, &8);
    dbg!(&s);

    let l = larger(&3, &2);
    dbg!(&l);
}

// In below example after returning reference of &country , country will be dropped so c will be dangling pointer
// as rust doesn't support it so to quic fix it, return ownership instead of returning reference.
pub fn get_country_name() ->  String {
    let country = String::from("Bangladesh");
    return country
}

// Here as i am always concerned about a and returning &a for always so no need to assign lifetime for b
pub fn  get_single_value <'l1>(a : &'l1 i32, b : &i32) -> &'l1 i32 {
    &a
}

// According to the first rule of lifetime Rust compiler will assign different lifetime for a and b
// but at the time of  returning &i32 borrow checker doesn't know which lifetime should be followed
// to inform it in compile time we should mention lifetime explicitly like below
pub fn larger <'l1 > (a : &'l1 i32, b : &'l1 i32) -> &'l1 i32 {
    if a > b {
        &a
    }else {
        &b
    }
}

