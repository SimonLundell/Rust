mod linalg;
mod layer_dense;

#[allow(dead_code)]
fn main() {
    // Sample input
    let input: Vec<Vec<f64>> = vec![vec![1.0 as f64, 2.0, 3.0, 2.5],
                                    vec![2.0 as f64, 5.0, -1.0, 2.0],
                                    vec![-1.5 as f64, 2.7, 3.3, -0.8]];
    
    let mut layer1: layer_dense::LayerDense = layer_dense::LayerDense::new(4, 5);
    let mut layer2: layer_dense::LayerDense = layer_dense::LayerDense::new(5, 2);
    
    // Passing previous layers output into next layer
    layer1.forward(&input);
    layer2.forward(&layer1.outputs);
    println!("{:?}", layer2.outputs);
}