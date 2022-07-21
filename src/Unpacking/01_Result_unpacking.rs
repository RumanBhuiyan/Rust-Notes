
pub fn run() {
    // Process - 01 : unpacking Result variants value using match expression
    let result = match get_result(true){
        Ok(value) => value,
        Err(message) => {println!("{:?}",message); 0}
    };
    println!("Ok() result : {:?}", result);

    let result = match get_result(false){
        Ok(value) => value,
        Err(message) => {println!("{:?}",message); 0}
    };
    println!("Err() result : {:?}", result);


    // Process - 02 : unpacking using ok(), err() built-in functions
    let result = get_result(true).ok().unwrap();
    println!("Ok() result : {}", result);

    let result = get_result(false).err().unwrap();
    println!("Err() result : {}", result);


    // Process - 03 : unpacking using unwrap() which returns only Ok() variants & panics otherwise
    let result = get_result(true).unwrap();
    println!("result : {:?}", result);
}

pub fn get_result(flag : bool) -> Result<i32, String>{
    if flag{
        Ok(76)
    }else{
        Err("Error: flag is false".to_string())
    }
}