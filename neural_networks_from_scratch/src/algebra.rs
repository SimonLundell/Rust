pub fn dot(m1: &Vec<Vec<f64>>, v2: &Vec<f64>) -> Vec<f64>
{
    let mut layer_outputs = vec![0.0 as f64; 3];
    
    for i in 0..layer_outputs.len()
    {
        for j in 0..v2.len()
        {
            layer_outputs[i] += v2[j] * m1[i][j];
        }
    }

    return layer_outputs;
}

pub fn add(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64>
{
    let mut ret = vec![0.0 as f64; v1.len()];
    for i in 0..ret.len()
    {
        ret[i] += v1[i] + v2[i];
    }

    return ret;
}