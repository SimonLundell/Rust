mod linalg;
mod layer_dense;
mod activation_relu;
mod spiral_dataset;

fn main() {
    // Sample input
    let input: Vec<Vec<f64>> = vec![vec![1.0 as f64, 2.0, 3.0, 2.5],
                                    vec![2.0 as f64, 5.0, -1.0, 2.0],
                                    vec![-1.5 as f64, 2.7, 3.3, -0.8]];
    
    let mut layer1: layer_dense::LayerDense = layer_dense::LayerDense::new(4, 5);
    // Prepare dataset
    let spiral = spiral_dataset::SpiralData::new(50,3);
    println!("{:?}", spiral.get_x());
    println!("{:?}", spiral.get_y());
    
    // Passing previous layers output into next layer
    layer1.forward(&input);

    let act: activation_relu::ActivationReLU = activation_relu::ActivationReLU::forward(vec![-1.0 as f64, 1.0, 2.0, 3.0, -1.0]);
}