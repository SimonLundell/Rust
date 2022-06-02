use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug)]
pub struct LayerDense
{
    val: Vec<Vec<f64>>
}

impl LayerDense
{
    pub fn new(n_inputs: i32, n_neurons: i32) -> LayerDense
    {
        let mut ret_layer = LayerDense{val: vec![]};
        let mut temp_v = vec![];
        for _i in 0..n_neurons
        {
            for _j in 0..n_inputs
            {
                let rng: f64 = thread_rng().sample(StandardNormal);
                temp_v.push(rng * 0.1);
            }
            ret_layer.val.push(temp_v.clone());
            temp_v.clear();
        }
    return ret_layer;
    }

    pub fn get_val(&self) -> &Vec<Vec<f64>>
    {
        return &self.val;
    }
}