#![allow(unused)]

enum Gender {
    Male,
    Female,
}

// Process - 01 : implementing methods for Gender enum directly
impl Gender {
    fn which_gender(&self) -> String{
        match self{
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string()
        }
    }
}

// Process - 02 : implementing methods for Gender through trait
trait GenderFuncs{
    fn greet(&self);
}

impl GenderFuncs for Gender {
    fn greet(&self) {
        println!("So you are a {}!", match *self {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
        });
    }
}

pub fn main(){
    let g = Gender::Male;

    println!("Gender of g : {}", g.which_gender());
    g.greet();
}