
enum Move
{
    Up,
    Down,
    Left,
    Right
}

fn movement(m: Move)
{
    match m
    {
        Move::Up => println!("Moving up"),
        Move::Down => println!("Moving down"),
        Move::Left => println!("Moving left"),
        Move::Right => println!("Moving right"),
    }
}

pub fn run()
{
    let up: Move = Move::Up;
    let down: Move = Move::Down;
    let left: Move = Move::Left;
    let right: Move = Move::Right;
    movement(up);
    movement(down);
    movement(left);
    movement(right);
}