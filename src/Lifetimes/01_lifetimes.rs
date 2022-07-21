/*
    lifetime is applicable for reference type input and reference type output
    &'static , here static is global scope lifetime which will live long entire program
    we can create our custom lifetime like &'a  &'b
*/

pub fn run(){
    let result = get_reference();
    dbg!(result);
    let b = Box::new(2);
    let c = Box::new(4);
    let _d =get_greater(&b,&c);
}

/*
     by using &'static here we are telling rust that don't drop "Happy coding" when you get }
     at end rather wait for finishing the entire program then drop off "Happy coding"
 */

pub fn get_reference()-> &'static str{
    "Happy coding"
}

/*
    after assigning reference to d if a and b are destroyed/dropped then d will point to which reference???
    that is called dangling pointer or dangling reference.
    that's why a,b should be alive till d will alive that means a,b,d should have same lifetime
    if we take only 1 reference input and provide another reference output then rust assign lifetime to that automatically
    but if we take multiple reference input then we have to specify lifetime lifetime manually like below
*/

pub fn get_greater<'l1 >(a : &'l1 i32 , b : &'l1 i32)-> &'l1 i32{
    if a>b {
        a
    }else {
        b
    }
}