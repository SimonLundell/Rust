pub struct Activation_ReLU
{
   pub outputs: Vec<f64>
}

impl Activation_ReLU
{
    pub fn forward(&mut self, inputs: Vec<f64>)
    {
        for i in 0..inputs.len()
        {
            if inputs[i] > 0.0
            {
                self.outputs.push(inputs[i]);
            }
            else
            {
                self.outputs.push(0.0);
            }
        }
    }
}