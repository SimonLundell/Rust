pub struct ActivationReLU
{
   pub outputs: Vec<f64>
}

impl ActivationReLU
{
    pub fn forward(inputs: Vec<f64>) -> ActivationReLU
    {
        let mut ret: Vec<f64> = vec![0.0; inputs.len()];
        for i in 0..inputs.len()
        {
            if inputs[i] > 0.0
            {
                ret[i] = inputs[i];
            }
        }

        return ActivationReLU{outputs: ret};
    }
}