#![allow(unused)]

/*
    Rust offers performing 4 types of operations in Arithmatic.
        i) checked operations (if overflows then return None else return Some(v))
        ii) wrapping operations (returns the remaining result that exists after overflow)
        iii) saturating operations (returns the closest result that can be represented by that data type)
        iv) overflowing operations (returns a tuple like (result_of_wrapping_operation, bool))
                                    where bool denotes whether overflow was occurred or not.

     N.B => for demo practise purpose i used just addition operation. you can also perform such other
            operations too like subtraction, multiplication, division , bitwise shift left, right etc
 */

pub fn main(){
    // checked operations
    let number = 200u8;
    match number.checked_add(50){
        Some(value) => println!("Result : {}", value),
        None => println!("Overflow occurred!!")
    }

    // wrapping operations
    println!("Result of 200 + 56 = {}",number.wrapping_add(57));

    // saturating operations
    println!("Result of 200 + 56 = {}",number.saturating_add(57));

    // overflowing operations
    println!("Result of 200 + 56 = {:?}",number.overflowing_add(57));
}
