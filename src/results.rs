/*
    ? operator can be used just inside a function which returns a Result<> type.
    If you use ? after a statement which returns a Result type then ? returns Ok() variant of
    that statement otherwise returns Err() variant. In Result<String, std::io::Error> we are telling
    Rust that Ok(r) variant r will be String and Err(e) variant e will be std::io::Error.
 */
use std::fs::File;
use std::io::Read;

pub fn run(){
    let file_content = get_file_content();
    dbg!(&file_content);

    let file_content2 = get_file_content2();
    dbg!(&file_content2);
}

pub fn get_file_content() -> Result<String, std::io::Error> {
    let mut file = File::open("./src/notes.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn get_file_content2() -> Result<String, std::io::Error>{
    let mut lines = String::new();
    match File::open("./src/notes2.txt"){
        Ok(mut file) => {
            file.read_to_string(&mut lines)?;
            Ok(lines)
        },
        Err(e) => Err(e)
    }
}