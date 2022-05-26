// Check the condition and act on the result


pub fn run()
{
    let age = 18;
    let check_id: bool = true;

    if age >= 21 && check_id
    {
        println!("Age bigger than 21");
    }
    else if check_id 
    {
        println!("Age less than 21");
    }

    // Shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("{}", is_of_age);
}