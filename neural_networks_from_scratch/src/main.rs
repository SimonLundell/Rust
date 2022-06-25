mod linalg;
mod layer_dense;
mod activation_softmax;
mod activation_relu;
mod spiral_dataset;

fn main() {
    // Sample input
    let dataset = spiral_dataset::SpiralData::new(100,3); // 100 samples, 3 classes

    let mut dense1: layer_dense::LayerDense = layer_dense::LayerDense::new(2, 3);
    let mut dense2: layer_dense::LayerDense = layer_dense::LayerDense::new(3, 3); // dense2 inputs must match dense1 outputs

    // Passing previous layers output into next layer
    dense1.forward(&dataset.get_x());
    let activation1: activation_relu::ActivationReLU = activation_relu::ActivationReLU::forward(dense1.outputs);

    dense2.forward(&activation1.outputs);
    let activation2: activation_softmax::ActivationSoftmax = activation_softmax::ActivationSoftmax::forward(dense2.outputs);

    println!("{:?}", activation2.outputs[1]); // 300 outputs from activation 2. We will get a "perfect" 1/3 distribution per neuron prediction of the input 2 values.
}