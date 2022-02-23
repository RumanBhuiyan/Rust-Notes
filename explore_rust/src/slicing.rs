// slicing [start_index..end_index] takes elements from start_index to end_index-1
pub fn run(){
    let numbers = [1,2,3,4,5];

    dbg!(&numbers[0..2]); // [1,2] excluding index 2
    dbg!(&numbers[0..=2]); // [1,2,3] including index 2
    dbg!(&numbers[0..]); // [1,2,3,4,5] from stat_index to last_index
    dbg!(&numbers[..3]); // [1,2,3] from stat_index to index 2
    dbg!(&numbers[..=3]); // [1,2,3,4] from stat_index to index 3
}