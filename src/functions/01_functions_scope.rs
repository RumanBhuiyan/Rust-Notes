/*
    function shadowing is valid for different scope, You can use any function before declaration
    if that function exists in current scope(global scope or main scope or local scope)

    f1() firstly searches this function in current scope if not found then searches in global scope
 */

pub fn f1(){
    println!("global scoped f1 get called!!")
}

pub fn run(){
    f1();
    {
        f1();
        pub fn f1(){
            println!("inner main f1 get called!!")
        }
    }

    pub fn f1(){
        println!("main scoped f1 get called!!")
    }
}

/* N.B => don't place ; at the end of last statement of a function. If you do so then Rust won't
treat that as a returning statement and won't return that statement eventually. Rather Rust will
return () an empty tuple as default return statement of any function. So you have 2
option. Either return statement like 'return <statement>;' or '<statement>'. If you return
anything from a function in first approach then it's not Rusty approach. So follow the second one.*/