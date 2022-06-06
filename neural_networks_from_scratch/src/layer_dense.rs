use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Debug)]
pub struct LayerDense
{
    weights: Vec<Vec<f64>>,
    biases: Vec<Vec<f64>>
}

impl LayerDense
{
    pub fn new(n_inputs: i32, n_neurons: i32) -> LayerDense
    {
        let ret_layer = LayerDense{weights: LayerDense::make_rng(n_inputs, n_neurons), 
                                    biases: vec![vec![0.0 as f64; n_neurons as usize]]};
        
        return ret_layer;
    }

    pub fn get_weights(&self) -> &Vec<Vec<f64>>
    {
        return &self.weights;
    }

    pub fn get_biases(&self) -> &Vec<Vec<f64>>
    {
        return &self.biases;
    }

    pub fn make_rng(n_inputs: i32, n_neurons: i32) -> Vec<Vec<f64>>
    {
        let mut ret = vec![];
        let mut temp_v = vec![];
        for _i in 0..n_neurons
        {
            for _j in 0..n_inputs
            {
                let rng: f64 = thread_rng().sample(StandardNormal);
                temp_v.push(rng * 0.1);
            }
            ret.push(temp_v.clone());
            temp_v.clear();
        }
        return ret;
    }
}