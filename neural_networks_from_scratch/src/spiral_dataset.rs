use rand::prelude::*;
use rand_chacha::ChaChaRng;
use crate::linalg;

pub struct SpiralData
{
    pub x: Vec<Vec<f64>>,
    pub y: Vec<i8>
}

impl SpiralData
{
    pub fn new(points: usize, classes: usize) -> SpiralData
    {
        let vector_length: usize = points * classes;
        let mut x: Vec<Vec<f64>> = vec![vec![0.0; 2]; vector_length];
        let mut y: Vec<i8> = vec![0; vector_length];

        let mut rng = ChaChaRng::seed_from_u64(2); // Any seed ok, pseudo-random
        let mut rng_vec: Vec<f64> = vec![0.0 as f64; points];

        for i in 0..points
        {
            rng_vec[i] = (rng.gen_range(-100..100) as f64 / 100.0) * 0.2;
        }

        for class_number in 0..classes
        {
            let r: Vec<f64> = SpiralData::linspace(0.0, 1.0, points);
            let t: Vec<f64> = linalg::vector_add(&SpiralData::linspace(class_number as f64 * 4.0, (class_number + 1) as f64 * 4.0, points), &rng_vec);
            
            for point in class_number*points..(class_number+1)*points
            {
                x[point][0] = r[point % 10] * (t[point % 10] * 2.5).sin();
                x[point][1] = r[point % 10] * (t[point % 10] * 2.5).cos();
                y[point] = class_number as i8;
            }
        }

        return SpiralData{x: x, y: y};
    }

    fn linspace(start: f64, stop: f64, size: usize) -> Vec<f64>
    {
        let mut ret: Vec<f64> = vec![start; size];
        let mut temp: f64 = start as f64;
        
        for i in 1..size
        {
            temp += stop / (size - 1) as f64;
            ret[i] = temp;
        }

        return ret;
    }

    pub fn get_x(&self) -> Vec<Vec<f64>>
    {
        return self.x.clone();
    }
    
    pub fn get_y(&self) -> Vec<i8>
    {
        return self.y.clone();
    }
}