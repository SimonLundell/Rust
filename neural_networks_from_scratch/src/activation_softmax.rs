pub struct ActivationSoftmax
{
   pub outputs: Vec<Vec<f64>>
}

impl ActivationSoftmax
{
    pub fn forward(inputs: Vec<Vec<f64>>) -> ActivationSoftmax
    {
        let e = std::f64::consts::E;
        let reduced_inputs: Vec<Vec<f64>> = ActivationSoftmax::sub_max(&inputs);

        let mut output: Vec<Vec<f64>> = vec![vec![0.0 as f64; reduced_inputs[0].len()]; reduced_inputs.len()];
       
        for i in 0..output.len()
        {
            for j in 0..output[i].len()
            {
                output[i][j] = e.powf(reduced_inputs[i][j]);
            }
        }

        let mut norm_base: Vec<f64> = vec![0.0 as f64; output.len()];

        for i in 0..output.len()
        {
            norm_base[i] = output[i].iter().sum();
        }

        let mut norm_values: Vec<Vec<f64>> = vec![vec![0.0 as f64; output[0].len()]; output.len()];

        for i in 0..output.len()
        {
            for j in 0..output[i].len()
            {
                norm_values[i][j] = output[i][j] / norm_base[i];
            }
        }

        return ActivationSoftmax{outputs: norm_values};
    }
        
    fn sub_max(inputs: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
    {
        let mut ret: Vec<Vec<f64>> = vec![vec![0.0 as f64; inputs[0].len()]; inputs.len()];

        for i in 0..inputs.len()
        {
            for j in 0..inputs[i].len()
                {
                    let max = inputs[i].iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    ret[i][j] = inputs[i][j] - max;
                }
        }    

        return ret;
    }
}