use std::collections::HashMap;
fn main() {
	
   let mut state_codes = HashMap::new();
	
   state_codes.insert("KL","Kerala");
   state_codes.insert("MH","Maharashtra");
	
   println!("size of map is {}",state_codes.len());

	// checking a  value
	  match state_codes.get(&"KL") {
      Some(value)=> {
         println!("Value for key KL is {}",value);
      }
      None => {
         println!("nothing found");
      }
   }

   // iterating using loop
	for (key, val) in state_codes.iter() {
      println!("key: {} val: {}", key, val);
   }

   // checking whether a key exists or not
	if state_codes.contains_key(&"KL") {
      println!("found key");
   }
	
}