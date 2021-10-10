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