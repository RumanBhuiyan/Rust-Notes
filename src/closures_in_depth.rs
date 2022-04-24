/* general functions can't be returned from inside a function but closure can be. In case of returning closure,
 3 types of syntax can be used as return statement : impl Fn(), impl FnMut(), impl FnOnce()
 you just use Fn() in all case and accomplish your task inside the function then compiler will inform you
 what to use Fn() or FnMut() or FnOnce(). impl Fn() trait bound is used as return statement as closure is returned
 from parent function and it has to capture dynamic values from that function and bind those values with returning closure.
 Fn() takes ownership immutably but FnMut() receives ownership mutably
 move keyword moves ownership of the parent function's resources that have been used inside closure to the closure. */

pub fn run(){
    let prefix = || String::from("Md. ");
    let name = get_name(prefix);
    println!("Actual name : {}",name(String::from("Roman")));

    let mut add = get_func(10);
    println!("After adding 25 : {}", add(25));
    println!("After adding 35 : {}", add(35));
}

pub fn get_name(func : fn() -> String) -> impl Fn(String) -> String{
    // move moves ownership of func to actual_name() from get_name()
    let actual_name = move |name : String| -> String {
        let mut full_name = String::new();
        full_name.push_str(func().as_str());
        full_name.push_str(name.as_str());
        return full_name;
    };
    actual_name
}

pub fn get_func(initial : i32) -> impl FnMut(i32) -> i32 {
    let mut summation = initial;
    println!("Initial value = {}",initial);
    // move moves ownership of summation to get_sum() from get_func()
    let get_sum = move | number : i32| -> i32{
        summation +=  number;
        summation
    };
    get_sum
}