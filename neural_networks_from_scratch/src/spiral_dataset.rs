pub struct SpiralData
{
    pub x: Vec<f64>,
    pub y: Vec<f64>
}

impl SpiralData
{
    pub fn get_x(&self) -> &Vec<f64>
    {
        return &self.x;
    }
    pub fn get_y(&self) -> &Vec<f64>
    {
        return &self.y;
    }
}