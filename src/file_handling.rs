// article : https://www.tutorialspoint.com/rust/rust_file_input_output.htm
use std::io::Read; // contains read_to_string() method and others
use std::io::Write; // contains write_all() method and others
use std::fs;

pub fn run(){
    // reading file
    let mut file = fs::File::open("src/stack_heap.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    dbg!(contents);

    // Writing to a file
    // create() destroy existing file's content and create new one
    let mut writing_file =  fs::File::create("src/notes.txt").unwrap();
    writing_file.write_all("Hello world!\n".as_bytes()).unwrap();
    writing_file.write_all("Happy Coding!\n".as_bytes()).unwrap();

    // writing to a file in append mode
    let mut my_file = fs::OpenOptions::new().append(true).open("src/notes.txt").unwrap();
    my_file.write_all("Good morning!!!\n".as_bytes()).unwrap();

    // Deleting file
    // fs::remove_file("src/notes.txt").unwrap();
}