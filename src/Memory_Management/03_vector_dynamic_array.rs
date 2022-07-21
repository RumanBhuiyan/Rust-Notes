// Look how the vector's capacity is being increased!! re-allocation isn't performed until
// len() > capacity(), if len() > capacity then double space of current length is taken from heap
// then items are copied there. That's why heap operations are costly.
pub fn run(){
    let mut numbers = vec![0;0];

    println!("After initialization, numbers.len() = {} numbers.capacity() = {}",
             numbers.len(), numbers.capacity());

    numbers.push(12);
    println!("After pushing 12, numbers.len() = {} numbers.capacity() = {}",
             numbers.len(), numbers.capacity());

    numbers.push(13);
    numbers.push(14);
    numbers.push(15);
    println!("After pushing 13, 14, 15, numbers.len() = {} numbers.capacity() = {}",
             numbers.len(), numbers.capacity());

    numbers.push(16);
    println!("After pushing 16, numbers.len() = {} numbers.capacity() = {}",
             numbers.len(), numbers.capacity());
}