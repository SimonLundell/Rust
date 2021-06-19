#[derive(Debug)]
struct Position {
    x:i32,
    y:i32,
}


fn main() {
    let x = 3; // i32 = signed 32 bit, u32 = unsigned
    let y = &x;

    let pos = Position {x:88, y:19}; // initializing position with variable values, mandatory.

    println!("My position is: {}, {}", pos.y, pos.x);

    println!("My debug position is: {:#?}", pos); //  This print way can only be used to debug what the values are. Requires #[derive(Debug)] , # is for pretty-print

    println!("Hello {}, world!", x*y);
}
