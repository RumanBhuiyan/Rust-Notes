fn main() {
	let arr:[i32;4] = [10,20,30,40];

	for x in &arr{
		println!("{} ",x);
	}

   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}