#![allow(unused)]

pub fn main() {
    // Process - 01 : unpacking Option variants value
    let result = match get_result(true){
        Some(value) => value,
        None => 0
    };
    println!("Some() result : {:?}", result);

    let result = match get_result(false){
        Some(value) => value,
        None => 0
    };
    println!("None result : {:?}", result);


    // Process - 02 : unpacking after checking is_some() , is_none() built-in functions
    let result = get_result(true);
    let mut value : i32;

    if result.is_some(){
        value = result.unwrap()
    }else{
        value = 0;
    }

    println!("Some() result : {:?}",value);

    let result = get_result(false);
    let mut value : i32;

    if result.is_none(){
        value =  0;
    }else{
        value = result.unwrap();
    }

    println!("None result : {:?}",value);


    // Process - 03 : unpacking using unwrap() which returns Some() & panics for None
    let result = get_result(true).unwrap();
    println!("Some() result: {:?}", result);
}

pub fn get_result(flag : bool) -> Option<i32>{
    if flag{
        Some(76)
    }else{
        None
    }
}