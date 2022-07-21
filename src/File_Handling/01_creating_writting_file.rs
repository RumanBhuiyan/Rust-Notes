#![allow(unused)]

use std::io::{Write};
use std::fs::{File};

pub fn run() {
    let mut file = File::create("./src/File_practise_txt_files/test1.txt").unwrap();
    file.write_all("Hello world".as_bytes()).unwrap();
}
