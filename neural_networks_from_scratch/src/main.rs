mod algebra;

fn main() {
    // Sample first neuron
    let inputs = vec![vec![1.0 as f64, 2.0, 3.0, 2.5],
                        vec![2.0 as f64, 5.0, -1.0, 2.0],
                        vec![-1.5 as f64, 2.7, 3.3, -0.8]]; // Input from a neuron in a layer earlier

    // Each input has an associated unique weight
    let mut weights = vec![vec![0.2 as f64, 0.8, -0.5, 1.0], 
                    vec![0.5 as f64, -0.91, 0.26, -0.5], 
                    vec![-0.26 as f64, -0.27, 0.17, 0.87]];

    weights = algebra::matrix_transpose(weights);

    // Each neuron has a unique bias, offsets the value in a neuron
    let biases = vec![2.0 as f64, 3.0, 0.5];

    // Perform dot product, important to have weights first
    let mut outputs = algebra::matrix_dot(&inputs, &weights);

    // Perform list addition (element by element addition)
    for i in 0..outputs.len()
    {
        outputs[i] = algebra::vector_add(&outputs[i], &biases);
    }

    println!("{:?}", outputs);
}
