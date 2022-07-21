use std::io::{stdout, Write};

/*
    write() expects bytes of &[u8] that's why as_bytes() converts any static(&str) or
            dynamic(String) strings to a reference of bytes of strings. Printing anything to
            console using stdout().write() is more efficient.
 */

pub fn run() {
    stdout().write("Hello world".as_bytes()).unwrap();
}