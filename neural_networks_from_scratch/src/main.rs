mod linalg;
mod layer_dense;
mod activation_relu;
mod spiral_dataset;

fn main() {
    // Sample input
    let dataset = spiral_dataset::SpiralData::new(100,3);
    let x: Vec<Vec<f64>> = dataset.get_x();
    let y = dataset.get_y();

    let mut layer1: layer_dense::LayerDense = layer_dense::LayerDense::new(2, 5);

    // Passing previous layers output into next layer
    layer1.forward(&x);

    let act: activation_relu::ActivationReLU = activation_relu::ActivationReLU::forward(layer1.outputs);
    println!("{:?}", act.outputs);
}