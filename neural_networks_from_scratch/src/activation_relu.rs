pub struct ActivationReLU
{
   pub outputs: Vec<Vec<f64>>
}

impl ActivationReLU
{
    pub fn forward(inputs: Vec<Vec<f64>>) -> ActivationReLU
    {
        let mut ret: Vec<Vec<f64>> = vec![vec![0.0; inputs[0].len()]; inputs.len()];
        for i in 0..inputs.len()
        {
            for j in 0..inputs[i].len()
            {
                if inputs[i][j] > 0.0
                {
                    ret[i][j] = inputs[i][j];
                }
            }
        }

        return ActivationReLU{outputs: ret};
    }
}