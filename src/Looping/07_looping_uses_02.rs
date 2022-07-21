#![allow(unused)]
pub fn run(){
    let mut nums = [1,2,3,4,5];

    get_double(&mut nums);

    for index in 0..5{
        println!("{}", nums[index])
    }
}
pub fn get_double(numbers : &mut [i32; 5]){
    for i in 0..5{
        // both of the line below does the same job
        // (*numbers)[i] *= 2
        numbers[i] *= 2;
    }
}
