#[derive(Clone)]
struct Vec
{
    x:f64,
    y:f64,
    z:f64
}

impl std::ops::Add for Vec
{
    type Output = Vec;

    fn add(self, v2: Vec) -> Vec
    {
        let ret: Vec = Vec
        {
            x: self.x + v2.x, 
            y: self.y + v2.y,
            z: self.z + v2.z
        };
        println!("{} {} {}", ret.x, ret.y, ret.z);

        return ret;
    }
}

impl std::ops::Sub for Vec
{
    type Output = Vec;
    
    fn sub(self, v2: Vec) -> Vec
    {
        let ret: Vec = Vec
        {
            x: self.x - v2.x, 
            y: self.y - v2.y,
            z: self.z - v2.z
        };
        println!("{} {} {}", ret.x, ret.y, ret.z);

        return ret;
    }
}

impl std::ops::Mul for Vec
{
    type Output = Vec;

    fn mul(self, v2: Vec) -> Vec
    {
        let ret: Vec = Vec
        {
            x: self.x * v2.x,
            y: self.y * v2.y,
            z: self.z * v2.z
        };
        println!("{} {} {}", ret.x, ret.y, ret.z);

        return ret;
    }
}

impl Vec
{
    fn dot(&self, v2: &Vec) -> f64
    {
        return self.x*v2.x + self.y*v2.y + self.z*v2.z;
    }


}

fn main()
{
    let vector1 = Vec{x:1.0,y:1.0,z:1.0};
    let vector2 = Vec{x:4.0,y:2.0,z:1.0};
    let vector3 = vector1.clone() + vector2.clone();
    let d = vector3.dot(&vector1);
    println!("{}", d);
    println!("{} {} {}", vector2.x, vector2.y, vector2.z);

}