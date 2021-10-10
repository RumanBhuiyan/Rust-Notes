# Rust-Notes
I explored Rust little bit and wrote down whatever i learnt

> ## print formats
```rust
      fn main() {

        // println! is macro not function
        println!(); // prints just a newline
        println!("hello ");//output:  hello
        println!("format {} arguments", "some"); //output: format some arguments
    }
```

> ## variables
```rust
      fn main() {

          // integer data tyep
          // integer -: i8 , i16, i32, i64 , i128
          // unsigned integer : u8, u16, u32 , u64, u128
          let age = 24 ; // Rust will detect it as  i32 by default
          let digit : i8 = 5 ; // telling Rust to allocate 8 bit for integer
          let number : i32 = 180 ; // telling Rust to allocate 32 bit for integer

          println!("Age : {} digit : {} number : {}",age,digit,number);


          // float data type
          // float : f32 , f64
          let temperature = 33.5; // f64 by default for double precision
          let area :f32 = 80.5 ;
          let pi : f64 = 3.1416 ;

          println!("temperature : {} are : {} pi : {}",temperature,area,pi);

          // boolean data type
          let is_sunday = false; // detects it as boolean automatically
          let is_saturday : bool = true; // telling Rust to take it as boolean

          println!("isSunday : {} isSaturday : {}",is_sunday,is_saturday);

          // character type
          let keep = 'R';
          let keep_char : char = 'r' ;

          println!("keep : {} keep_char : {}",keep,keep_char);

          const USER_LIMIT:i32 = 100;    // Declare a integer constant
          const PI:f32 = 3.14;           //Declare a float constant

          println!("user limit is {}",USER_LIMIT);  //Display value of the constant
          println!("pi value is {}",PI);

          // Rust allows redeclaring variable but doesn't support auto-type cast
          let name : &str= "Ruman";
          println!("Name : {}",name);

          let name  = name.len();
          println!("name : {}",name);

          // string object
          let empty_string = String::new();
          println!("length is {}",empty_string.len());

          let content_string = String::from("TutorialsPoint");
          println!("length is {}",content_string.len());

}
```

> ## Data types
```rust
      fn main() {

          // integer data tyep
          // integer -: i8 , i16, i32, i64 , i128
          // unsigned integer : u8, u16, u32 , u64, u128
          let age = 24 ; // Rust will detect it as  i32 by default
          let digit : i8 = 5 ; // telling Rust to allocate 8 bit for integer
          let number : i32 = 180 ; // telling Rust to allocate 32 bit for integer

          println!("Age : {} digit : {} number : {}",age,digit,number);


          // float data type
          // float : f32 , f64
          let temperature = 33.5; // f64 by default for double precision
          let area :f32 = 80.5 ;
          let pi : f64 = 3.1416 ;

          println!("temperature : {} are : {} pi : {}",temperature,area,pi);

          // boolean data type
          let is_sunday = false; // detects it as boolean automatically
          let is_saturday : bool = true; // telling Rust to take it as boolean

          println!("isSunday : {} isSaturday : {}",is_sunday,is_saturday);

          // character type
          let keep = 'R';
          let keep_char : char = 'r' ;

          println!("keep : {} keep_char : {}",keep,keep_char);

      }
```

> ## Conditional Statements 
```rust
      fn main(){
        let number = get_input();

        if number > 0 {
          println!("Positive Number");
        }else if number < 0 {
          println!("Negative Number");
        }else {
          println!("Zero");
        }
      }

      fn get_input() -> i32{

          let mut line  = String::new();
          std::io::stdin().read_line(&mut line).unwrap();
          let number : i32 = line.trim().parse().unwrap();
          return number ;
      }
```
> ## Looping
```rust
      fn main(){

        // while loop
        let mut counter = get_input();
        while counter > 0{
          println!("Hello world");
          counter -=1;
        }
      }

      fn get_input() -> i32{

          let mut line  = String::new();
          std::io::stdin().read_line(&mut line).unwrap();
          let number : i32 = line.trim().parse().unwrap();
          return number ;
      }
```
> ## functions
```rust
      fn main(){

          let first = get_input();
          let second = get_input();

          println!("Summation {}",get_sum(first,second));
          println!("Product is {}",get_product(first,second));
       }

       fn get_sum(a:i32,b:i32) -> i32{
           return a+b;
       }

       fn get_product(a:i32,b:i32) -> i32{
           return a*b;
       }

       fn get_input() -> i32{

           let mut line  = String::new();
           std::io::stdin().read_line(&mut line).unwrap();
           let number : i32 = line.trim().parse().unwrap();
           return number ;
 }
```
> ## Structures
```rust
      struct Employee {
          name:String,
          company:String,
          age:u32
       }

       fn main() {

          let emp1 = Employee {
             company:String::from("TutorialsPoint"),
             name:String::from("Mohtashim"),
             age:50
          };

          let emp2 = Employee{
             company:String::from("TutorialsPoint"),
             name:String::from("Kannan"),
             age:32
          };

          display(emp1);
          display(emp2);
       }


       fn display( emp:Employee){
          println!("Name is :{} company is {} age is 
          {}",emp.name,emp.company,emp.age);
       }
```
> ## Tuple
```rust
      fn main() {
          // decalring & initializing tuple
        let  sphere :(i32,f32) = (4,16.0);

        println!("{:?}",sphere);

        //destructuring variables
        let (radius,area) = sphere;

        println!("radius : {} area : {}",radius,area);
      }
```
> ## Array
```rust
      fn main() {
        let arr:[i32;4] = [10,20,30,40];

        for x in &arr{
          println!("{} ",x);
        }

         println!("array is {:?}",arr);
         println!("array size is :{}",arr.len());
      }
```

> ## vectors
```rust
      fn main() {

          let mut v = Vec::new();

          v.push(20);
          v.push(30);
          v.push(40);
          v.push(50);

          v.remove(0); // remove 0th index

          println!("size of vector is :{}",v.len());
          println!("{:?}",v);
          println!("{}",v.contains(&40));

           for i in &v {
             println!("{}",i);
          }
       }
```

> ## hashmap
```rust
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
```

> ## Reading user input
```rust
      fn main() {

          let first_number = get_input();
          let second_number = get_input();

          println!("Summation : {}",first_number+second_number);

      }

      fn get_input() -> i32{

          let mut line  = String::new();
          std::io::stdin().read_line(&mut line).unwrap();
          let number : i32 = line.trim().parse().unwrap();
          return number ;
      }
```
