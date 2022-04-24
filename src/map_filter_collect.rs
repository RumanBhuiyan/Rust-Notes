pub fn run(){
    let  numbers = vec![1,2,3,4,5];

    // map
    let product_of_two : Vec<i32> = numbers.iter().map(|x| x*2).collect();
    dbg!(&product_of_two);

    // filter works on an iterator and return another iterator
    let even_numbers = product_of_two.iter()
                                    .filter(|x| **x%2 ==0)
                                    .map(|x| *x)
                                    .collect::<Vec<i32>>();
    dbg!(&even_numbers);

    let even_numbers2 = product_of_two.iter()
                                    .filter(|&&x| x %2 ==0)
                                    .map(|&x|x)
                                    .collect::<Vec<i32>>();
    dbg!(&even_numbers2);
}