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