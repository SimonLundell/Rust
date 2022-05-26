
pub fn run()
{
    printstuff("Simon", "TheBest");
    let my_val: f32 = add(5.3, 4.6);
    println!("{}", my_val);


    // Closure (lambda). Can also use external variables
    let ext_val: i32 = 10;
    let lambd = |v1: i32, v2: i32| v1 + v2 + ext_val;

    println!("{}", lambd(24,32));
}

fn printstuff(name: &str, nickname: &str)
{
    println!("{} {}", name, nickname);
}

fn add(val1: f32, val2: f32) -> f32
{
    return val1 + val2;
}