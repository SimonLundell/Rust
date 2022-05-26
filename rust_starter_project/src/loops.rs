
pub fn run()
{
    let mut count = 0;

    while count < 50
    {
        if count % 3 == 0
        {
            println!("{} is divisible by 3", count);
        }
    count += 1;
    }

    for x in 0..10
    {
        if x % 4 == 0
        {
            println!("{} is divisible by 4", x);
        }
    }
}