// Structs : used to create custom data-types

// Traditional struct
struct Color
{
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct ColorT(u8, u8, u8);

// Advanced struct
struct Person
{
    first_name: String,
    last_name: String
}

impl Person
{
    fn new(first: &str, last: &str) -> Person
    {
        return Person{first_name: first.to_string(), last_name: last.to_string()};
    }

    fn get_fullname(&self) -> String
    {
        return format!("{} {}", self.first_name, self.last_name);
    }

    fn set_lastname(&mut self, last: &str) // to change self has to be mutable
    {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) // This function MOVES the names out of the struct, add & to all variables to keep in original struct
    {
        (self.first_name, self.last_name) // omit ; to have automatic return
    }
}

pub fn run()
{
    let mut c: Color = Color {red: std::u8::MAX,green: std::u8::MIN,blue: std::u8::MAX};

    println!("Red:{} Green:{} Blue:{}", c.red, c.green, c.blue);
    c.blue = 49;

    println!("Red:{} Green:{} Blue:{}", c.red, c.green, c.blue);
    
    let mut ct = ColorT(50,70,40);
    ct.0 = 200;

    println!("Red:{} Green:{} Blue:{}", ct.0, ct.1, ct.2);

    let mut me: Person = Person::new("Simon", "Lundell");
    println!("{} {}", me.first_name, me.last_name);
    me.set_lastname("Ge");
    println!("{}", me.get_fullname());

    let me_as_tup = me.to_tuple();
    println!("{:?}", me_as_tup);
    // println!("{}", me.get_fullname()); Cant be done because names are now in me_as_tup
}