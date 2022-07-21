pub fn main(){
    let test_case = get_int_input();

    for _ in 1..=test_case{
        let line = get_line_input();
        let mut digits : Vec<u32> = vec![];

        for character in line.chars(){
            let value :u32  = match character.to_digit(10){
                Some(v) => v,
                _ => 0 as u32
            };
            digits.push(value);
        }

        if digits[0] + digits[1] + digits[2] == digits[3] + digits[4] + digits[5]{
            println!("yes")
        } else{
            println!("no")
        }
    }
}

pub fn get_int_input() -> i32{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}

pub fn get_line_input() -> String{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

// another solution
// pub fn main(){
//     let test_case = get_int_input();
//
//     for _ in 1..=test_case{
//         let line = get_line_input();
//         let mut counter = 1;
//         let mut summation = 0;
//
//         for character in line.chars(){
//             let value :u32  = match character.to_digit(10){
//                 Some(v) => v,
//                 _ => 0 as u32
//             };
//             if counter <= 3{
//                 summation += value
//             }else{
//                 summation -= value
//             }
//             counter += 1;
//         }
//         if summation == 0{
//             println!("yes")
//         } else{
//             println!("no")
//         }
//     }
// }
