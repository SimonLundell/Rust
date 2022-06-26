mod linalg;
mod layer_dense;
mod activation_softmax;
mod activation_relu;
mod loss_categorical_crossentropy;
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

    println!("{:?}", activation2.outputs.len()); // 300 outputs from activation 2. We will get a "perfect" 1/3 distribution per neuron prediction of the input 2 values.

    let loss: loss_categorical_crossentropy::LossCategoricalCrossentropy = loss_categorical_crossentropy::LossCategoricalCrossentropy::calculate(activation2.outputs, dataset.get_y());
    println!("{:?}", loss.loss);

    // First loss calculation
    // let test_output = vec![0.7 as f64, 0.1, 0.2];
    // let target_output = vec![1.0 as f64, 0.0, 0.0];

    // // As prediction confidence gets lower, loss gets higher!
    // let loss = -(test_output[0].ln()); // * target_output[0] + test_output[1].ln() * target_output[1] + test_output[2].ln() * target_output[2]); Can be abstracted away due to 0 and 1 multiplication
    // println!("{}", loss); 

    // let softmax_outputs = vec![vec![0.7 as f64, 0.1, 0.2],
    //                             vec![0.1 as f64, 0.5, 0.4],
    //                             vec![0.02 as f64, 0.9, 0.08]];

    // let class_targets = vec![0, 1, 1];
    // let mut losses = vec![0.0 as f64; class_targets.len()];
    // for i in 0..class_targets.len()
    // {
    //     losses[i] = -softmax_outputs[i][class_targets[i]].ln();
    // }
    // println!("{:?}",losses);

    // let sum: f64 = losses.iter().sum();
    // let mean: f64 = sum / losses.len() as f64;
    // println!("{:?}", mean)
}