pub fn run(){
    let  numbers = vec![1,2,3,4,5];
    // iterator returns a reference/memory address. so if you receive it
    // by x then you can get its value by x or *x but if you accept it
    // by &x then you can access its value by x not *x
    // map works on an iterator and returns another iterator
    let product_of_two = numbers.iter().map(|x| x*2).collect::<Vec<i32>>();

    // filter works on an iterator and return another iterator
    let even_numbers = product_of_two.iter().filter(|&&x| x%2 ==0).map(|&x|x).collect::<Vec<i32>>();
    let even_numbers2 = product_of_two.iter().filter(|x| **x %2 ==0).map(|x|*x).collect::<Vec<i32>>();

    dbg!(even_numbers);
    dbg!(even_numbers2);
}