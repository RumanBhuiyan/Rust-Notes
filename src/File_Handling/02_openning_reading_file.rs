use std::io::{Read, Write};
use std::fs::{File};

pub fn run() {
    let mut file_contents = String::new();

    let mut file = File::open("./src/File_practise_txt_files/test1.txt").unwrap();

    file.read_to_string(&mut file_contents).unwrap();
    println!("{:?}", file_contents);
}
