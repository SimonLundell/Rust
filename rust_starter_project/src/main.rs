#[derive(Debug)]
struct Position 
{
    x:i32,
    y:i32,
}

impl Position // method belonging to struc Position
{
    fn update_y(&mut self)
    {
        self.y += 22;
    }
}

fn update_x(pos: &mut Position, val: i32) // variable name: datatype
{
    pos.x = val;
}


fn main() 
{
    let x = 3; // i32 = signed 32 bit, u32 = unsigned
    let y = &x;

    let pos = Position {x:88, y:19}; // initializing position with variable values, mandatory.
    //let pos2 = pos; // this moves pos -> pos2. Because of this, below variables using pos will not work. Have to use pos2
    let _pos2 = &pos; // makes pos2 a "borrow (reference)" of pos. This works only on immutable data (which it is by default). Multiple pointers to the same value.
    
    let mut _pos3 = Position {x:40,y:20};
    
    update_x(&mut _pos3, 50); // same as line below. have to make _pos3 mutable and the argument taken has to be mutable.
    
    Position::update_y(&mut _pos3); // globally accessible functin update_y but with "namespace" required before.
    
    _pos3.x = 60;

    println!("My pos3 initial x is 40, and after that my pos3 x is {}. My pos3.y is {}",_pos3.x, _pos3.y);
    _pos3.update_y();
    println!("after update y, pos3.y is {}", _pos3.y);
    println!("My position is: {}, {}", pos.y, pos.x);

    println!("My debug position is: {:#?}", pos); //  This print way can only be used to debug what the values are. Requires #[derive(Debug)] , # is for pretty-print

    println!("Hello {}, world!", x*y);
}
