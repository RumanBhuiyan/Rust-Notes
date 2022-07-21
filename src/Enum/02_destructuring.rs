#[derive(Debug)]
pub enum Message {
    Content{ id : i32 }
}

pub fn run(){
    let m1 = Message::Content{ id : 5 };
    let m2 = Message::Content{ id : 6 };
    let m3 = Message::Content{ id : -1 };

    inform_range(&m1);
    inform_range(&m2);
    inform_range(&m3);

    inform_range2(&m1);
    inform_range2(&m2);
    inform_range2(&m3);
}

pub fn inform_range(m :&Message){
    // variable @ range , checks the value bound in left-side satisfies the range at right of @
 match m {
     Message::Content{id : _id_value @ 1..=5 } => println!("id is within range 1..5"),
     Message::Content{id} if id > &5 => println!("id greater than 5"),
     _ => println!("Range isn't defined")
  }
}

pub fn inform_range2(m :&Message){
    // destructuring value in left side using pattern matching mechanism of Rust
  let Message::Content {id : id_value} = m;
     match id_value{
         1..=5 => println!("id is withing range 1..5"),
         y if  y > &5 =>  println!("id is greater than 5"),
         _ =>  println!("range isn't defined"),
     }

}