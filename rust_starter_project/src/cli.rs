
pub fn run()
{
    let args: Vec<String> = std::env::args().collect(); // get arguments from commandline

    println!("{:?}", args);
    
    if args.len() > 1
    {
        if args[1] == "Hello"
        {
            println!("Well hey there!");
        }
    }
}