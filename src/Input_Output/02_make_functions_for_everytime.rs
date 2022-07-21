/* N.B => don't place ; at the end of last statement of a function. If you do so then Rust won't
treat that as a returning statement and won't return that statement eventually. So you have 2
option. Either return statement like 'return <statement>;' or '<statement>'. If you return
anything from a function in first approach then it's not Rusty approach. So follow the second one.*/

pub fn get_int_input() -> i32{
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}

pub fn get_line_input() -> String{
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}