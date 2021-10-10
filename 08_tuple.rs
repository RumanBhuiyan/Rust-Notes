fn main() {
    // decalring & initializing tuple
	let  sphere :(i32,f32) = (4,16.0);

	println!("{:?}",sphere);

	//destructuring variables
	let (radius,area) = sphere;

	println!("radius : {} area : {}",radius,area);
}