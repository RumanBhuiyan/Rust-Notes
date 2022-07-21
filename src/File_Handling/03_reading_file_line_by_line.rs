use std::io::{Read, stdin, Write, BufReader, BufRead};
use std::fs::{File};

pub fn run() {
    let mut file_contents = String::new();
    let mut file = File::open("./src/File_practise_txt_files/test1.txt").unwrap();

    // Process - 01 : loading whole file contents into a mutable variable and reading line by line
    file.read_to_string(&mut file_contents).unwrap();
    for line in file_contents.lines(){
        println!("{}",line);
    }

    // Process - 02: reading file line by line using BufferReader
    let file = File::open("./src/File_practise_txt_files/test1.txt").unwrap();
    let file_buffer = BufReader::new(file);
    for line in file_buffer.lines(){
        println!("{}",line.unwrap());
    }
}