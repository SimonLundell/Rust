mod algebra;
mod layer_dense;

fn main() {
    let test: layer_dense::LayerDense = layer_dense::LayerDense::new(4, 4);
    println!("{:?}", test.get_weights());
    println!("{:?}", test.get_biases());
    // Sample first neuron
    let x = vec![vec![1.0 as f64, 2.0, 3.0, 2.5],
                        vec![2.0 as f64, 5.0, -1.0, 2.0],
                        vec![-1.5 as f64, 2.7, 3.3, -0.8]]; // Input from a neuron in a layer earlier

    // Each input has an associated unique weight
    let mut weights = vec![vec![0.2 as f64, 0.8, -0.5, 1.0], 
                    vec![0.5 as f64, -0.91, 0.26, -0.5], 
                    vec![-0.26 as f64, -0.27, 0.17, 0.87]];
    // Second layer
    let mut weights2 = vec![vec![0.1 as f64, -0.14, 0.5], 
                    vec![-0.5 as f64, 0.12, -0.33,], 
                    vec![-0.44 as f64, 0.73, -0.13]];
    
    weights = algebra::matrix_transpose(weights);
    weights2 = algebra::matrix_transpose(weights2);
    // Each neuron has a unique bias, offsets the value in a neuron
    let biases = vec![2.0 as f64, 3.0, 0.5];
    let biases2 = vec![-1.0 as f64, 2.0, -0.5];

    // Perform dot product, important to have weights first
    let mut layer1_outputs = algebra::matrix_dot(&x, &weights);

    // Perform list addition (element by element addition)
    for i in 0..layer1_outputs.len()
    {
        layer1_outputs[i] = algebra::vector_add(&layer1_outputs[i], &biases);
    }

    // Second layer with first layer as input
    let mut layer2_outputs = algebra::matrix_dot(&layer1_outputs, &weights2);
    
    for i in 0..layer2_outputs.len()
    {
        layer2_outputs[i] = algebra::vector_add(&layer2_outputs[i], &biases2);
    }

    println!("{:?}", layer2_outputs);
}
