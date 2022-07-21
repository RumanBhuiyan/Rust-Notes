#![allow(unused)]

/*
    String is not an array of characters rather an array of bytes which represent characters.
    There are 3 types of strings in Rust.
        i) String (Vec<u8>) it's heap allocated growable sized.
        ii) string slice (&[u8]) ; reference to a borrowed text/strings owned by other
        iii) string literal ; It's a pre-allocated string that stays in ram and created when programs
                                get started executing

    In the example below, country is String, state_code is string slice and another_country is
    string literal.
    country is Vec<u8>
    state_code is a fat pointer which has 2 properties. pointer of data buffer and size.
    another_country is a string literal that is created in RAM when program executions started.
 */

pub fn run(){
    let country = "Bangladesh".to_string();
    let state_code = &country[country.len()-4..];
    let another_country = "Canada";
}