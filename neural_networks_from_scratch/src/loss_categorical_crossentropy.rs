pub struct LossCategoricalCrossentropy
{}

impl LossCategoricalCrossentropy
{
    pub fn calculate(output: Vec<Vec<f64>>, y: Vec<i8>) -> f64
    {
        let sample_losses: Vec<f64> = LossCategoricalCrossentropy::forward(output, y);
        
        let sum: f64 = sample_losses.iter().sum();
        let data_loss: f64 = sum / sample_losses.len() as f64;

        return data_loss;
    }

    fn forward(y_pred: Vec<Vec<f64>>, y_true: Vec<i8>) -> Vec<f64>
    {
        let e = std::f64::consts::E;
        let samples = y_pred.len();
        let y_pred_clipped = LossCategoricalCrossentropy::clip(y_pred, 1.0*e.powf(-7.0));
        let mut correct_confidences = vec![0.0 as f64; samples];

        // Needs logic if y_true is onehot encoded
        for i in 0..samples
        {
            correct_confidences[i] = -y_pred_clipped[i][y_true[i] as usize].ln();
        }

        return correct_confidences;
    }

    fn clip(mut matrix: Vec<Vec<f64>>,value: f64) -> Vec<Vec<f64>>
    {
        for i in 0..matrix.len()
        {
            for j in 0..matrix[i].len()
            {
                if matrix[i][j] < value
                {
                    matrix[i][j] = value;
                }
                else if matrix[i][j] > 1.0 - value
                {
                    matrix[i][j] = 1.0 - value;
                }
            }
        }

        return matrix;
    }
}
