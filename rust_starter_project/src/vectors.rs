// Vectors are re-sizeable arrays

pub fn run()
{
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers); // :? is debug mode
    println!("Get single value {}", numbers[0]);
    println!("Size of vector {}", numbers.len());
    println!("Memory used by vector {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2]; // & so it can reference numbers and return correct slice
    println!("Slice is {:?}", slice);

    numbers.push(6);
    numbers.push(7);
    numbers.pop();
    println!("{:?}", numbers);

    for x in numbers.iter()
    {
        println!("{}", x+100);
    }

    for i in numbers.iter_mut()
    {
        *i *= 2; // Need to de-reference the iterator (pointer)
    }
    println!("{:?}", numbers);
}