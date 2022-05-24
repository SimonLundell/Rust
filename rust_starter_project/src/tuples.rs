
pub fn run() 
{
    let person: (&str, &str, i8) = ("Simon", "Lundell", 32);

    println!("{} last name is {} and he is {}", person.0, person.1, person.2);

}