#[allow(dead_code)]
pub fn matrix_dot(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    let mut ret = vec![];

    for i in 0..m1.len()
    {
        let mut row_result = vec![];
        let temp_v1 = extract_row(&m1,i);
        for j in 0..m2[0].len()
        {
            let temp_v2 = extract_column(m2, j);
            let dot_prod = vector_dot(&temp_v1, &temp_v2);
            row_result.push(dot_prod);
        }
        ret.push(row_result);
    }

    return ret;
}

#[allow(dead_code)]
pub fn matrix_transpose(m1: Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    let mut ret = vec![];

    for i in 0..m1[0].len()
    {
        let mut temp_v = vec![];
        for j in 0..m1.len()
        {
            temp_v.push(m1[j][i]);
        }
        ret.push(temp_v);
    }

    return ret;
}

#[allow(dead_code)]
pub fn matrix_add(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    let mut ret = vec![vec![0.0 as f64; m1[0].len()]; m1.len()];

    for i in 0..ret.len()
    {
        ret[i] = vector_add(&m1[i], &m2[i]);
    }

    return ret;
}

#[allow(dead_code)]
pub fn vector_dot(v1: &Vec<f64>, v2: &Vec<f64>) -> f64
{
    let mut layer_outputs = 0.0;
    
    for i in 0..v1.len()
    {
        layer_outputs += v1[i]*v2[i];
    }

    return layer_outputs;
}

#[allow(dead_code)]
pub fn vector_add(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64>
{
    let mut ret = vec![0.0 as f64; v1.len()];
    for i in 0..ret.len()
    {
        ret[i] += v1[i] + v2[i];
    }

    return ret;
}

#[allow(dead_code)]
pub fn extract_row(m: &Vec<Vec<f64>>, idx: usize) -> Vec<f64>
{
    return m[idx].clone();
}

#[allow(dead_code)]
pub fn extract_column(m: &Vec<Vec<f64>>, idx: usize) -> Vec<f64>
{
    let mut ret_v = vec![];
    for i in 0..m.len()
    {
        ret_v.push(m[i][idx]);
    }
    return ret_v;
}