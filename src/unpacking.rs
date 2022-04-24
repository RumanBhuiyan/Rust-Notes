pub fn run(){

    let k = (1,2,3);

    // Here Rust match patterns in left side and right side then assign values
    let (a,_,_) = k;
    let (b,..) = k;

    dbg!(a);
    dbg!(b);
    dbg!(k);

    // N.B => Here Rust also match pattern of w then binds value 24 to w
    let w = 24;
}