pub struct ActivationSoftmax
{
   pub outputs: Vec<Vec<f64>>
}

impl ActivationSoftmax
{
    pub fn forward(inputs: Vec<Vec<f64>>) -> ActivationSoftmax
    {
        let e = std::f64::consts::E;
        let mut output = vec![];
        let reduced_inputs: Vec<Vec<f64>> = ActivationSoftmax::sub_max(&inputs);

        for arr in reduced_inputs
        {
            let mut temp_arr = vec![];
            for val in arr
            {
                temp_arr.push(e.powf(val));
            }
            output.push(temp_arr); 
        }

        let mut norm_base: Vec<f64> = vec![];
        let mut norm_values: Vec<Vec<f64>> = vec![];

        for out in &output
        {
            norm_base.push(out.iter().sum());
        }

        for i in 0..output.len()
        {
            let mut temp_arr = vec![];
            for j in 0..output[i].len()
            {
                temp_arr.push(output[i][j]/norm_base[i])
            }
            norm_values.push(temp_arr);
        }

        return ActivationSoftmax{outputs: norm_values};
    }
        
    fn sub_max(inputs: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
    {
        let maximums: Vec<f64> = ActivationSoftmax::get_max(&inputs);
        let mut ret_matrix: Vec<Vec<f64>> = vec![];
        for i in 0..inputs.len()
        {
            let mut temp_arr = vec![];
            for j in 0..inputs[i].len()
                {
                    temp_arr.push(inputs[i][j] - maximums[i]);
                }
                ret_matrix.push(temp_arr);
        }    
        return ret_matrix;
    }

    fn get_max(inputs: &Vec<Vec<f64>>) -> Vec<f64>
    {
        let mut maximums: Vec<f64> = vec![];
        for arr in inputs
        {
            maximums.push(arr.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        }

        return maximums;
    }
}