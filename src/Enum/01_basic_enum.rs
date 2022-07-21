#![allow(unused)]

/*
    Rust doesn't support null keyword instead it takes None and None exists in Option<> enum
    if any function can return null/None then we should use Option<returning_value_type> type enum
    as return item
*/

enum Gender {
    Male,
    Female
}

fn print_gender(gender : Gender){
    match gender {
        Gender::Male => println!("Gender : Male"),
        Gender::Female => println!("Gender : Female")
    }
}

pub fn run(){
    let gender1 = Gender::Male;
    let gender2 = Gender::Female;

    print_gender(gender1);
    print_gender(gender2);
}