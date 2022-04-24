pub fn run(){
    // pattern and matching let's look how Rust unpack values by matching pattern
    let a = Some(5);

   match a {
        Some(50) => println!("first match :  50"),
        Some(val) => println!("a : {}",val),
        _ => println!("No match found")
   }

   /* pattern with ranges
    N.B => ranges in pattern only applicable for numeric and char type values */
    let mark = 100;
    match mark {
        80..=100 => println!("A+"),
        70..=79 => println!("A"),
        60..=69 => println!("A-"),
        50..=59 => println!("B"),
        40..=49 => println!("C"),
        _ => println!("Failed")
    }

    let character = 'a';
    match character {
        'A'..='Z' => println!("capital letter"),
        'a'..='z' => println!("small letter"),
        _ => println!("character unknown")
    }

}