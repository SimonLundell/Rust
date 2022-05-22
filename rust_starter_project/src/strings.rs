// let hello = "Hello" primitive string, Immutable fixed string somewhere in memory
// let mut hello = String::from("Hello ") growable heap-allocated string


pub fn run()
{
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    hello.push('W');
    println!("{}", hello);

    hello.push_str("orld!");
    println!("{}", hello);

    println!("Capacity in bytes: {}", hello.capacity());
    println!("Length: {}", hello.len());
    println!("Is empty: {}", hello.is_empty());
    println!("Contains: {}", hello.contains("World!"));
    println!("Replace: {}", hello.replace("World!", "There!"));

    hello.push_str(" Fuck You");
    // Loop by whitespace
    for word in hello.split_whitespace()
    {
        println!("{}", word)
    }
}