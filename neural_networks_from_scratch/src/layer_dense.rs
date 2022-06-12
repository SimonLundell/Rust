use crate::linalg;

use rand::prelude::*;
use rand_distr::StandardNormal;


pub struct LayerDense
{
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<Vec<f64>>,
    pub outputs: Vec<Vec<f64>>
}

#[allow(dead_code)]
impl LayerDense
{
    // Create new layer with initial value of rng, number of vectors as n_inputs and number of values as n_neurons
    // Equally big biases (all 0)
    pub fn new(n_inputs: i32, n_neurons: i32) -> LayerDense
    {
        let ret_layer = LayerDense{weights: LayerDense::make_rng(n_inputs, n_neurons), 
                                    biases: vec![vec![0.0 as f64; n_neurons as usize]; n_inputs as usize],
                                    outputs: vec![]};
        
        return ret_layer;
    }

    pub fn forward(&mut self, inputs: &Vec<Vec<f64>>)
    {
        self.outputs = linalg::matrix_add(&(linalg::matrix_dot(&inputs, &self.weights)), &self.biases);
    }

    pub fn make_rng(n_inputs: i32, n_neurons: i32) -> Vec<Vec<f64>>
    {
        let mut ret = vec![];
        let mut temp_v = vec![];
        for _i in 0..n_inputs
        {
            for _j in 0..n_neurons
            {
                let rng: f64 = thread_rng().sample(StandardNormal);
                temp_v.push(rng * 0.1);
            }
            ret.push(temp_v.clone());
            temp_v.clear();
        }
        return ret;
    }

    pub fn get_weights(&self) -> &Vec<Vec<f64>>
    {
        return &self.weights;
    }

    pub fn get_biases(&self) -> &Vec<Vec<f64>>
    {
        return &self.biases;
    }
}