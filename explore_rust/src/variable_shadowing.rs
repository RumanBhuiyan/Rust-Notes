pub fn run(){
    let amount = 15;

    {
        // this amount variable creates a shadow upon outer amount
        // after the execution of this block the amount variable is dropped from memory
        let amount = 20;
        println!("Amount : {}",amount);
    }
    // as inner amount is dropped of so now outer amount is accessible
    println!("Amount : {}",amount);
}