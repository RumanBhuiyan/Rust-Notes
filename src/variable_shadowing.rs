/* inner amount variable creates a shadow upon outer amount
 after the execution of this block inner amount variable is dropped from memory
  then outer amount will be accessible as shadow is gone*/

pub fn run(){
    let amount = 15;
    {
        let amount = 20;
        println!("Amount : {}",amount); // 20
    }
    println!("Amount : {}",amount); // 15
}