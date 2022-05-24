// Arrays - fixed arrays with same datatypes

pub fn run()
{
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers); // :? is debug mode
    println!("Get single value {}", numbers[0]);
    println!("Size of array {}", numbers.len());
    println!("Memory used by array {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2]; // & so it can reference numbers and return correct slice
    println!("Slice is {:?}", slice);
}