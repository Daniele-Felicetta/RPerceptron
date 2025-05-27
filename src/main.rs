mod perceptron;
mod types;
mod utils;
mod formulas;

use types::PerceptronInputData;
use types::TrainingData;
use perceptron::perceptron;

fn main() {
    let training = TrainingData{
        x: vec![[0,1],[0,0],[1,0],[1,1]],
        y: vec![0,1,0,1],
    };

    let percetron_input = PerceptronInputData{
        training_data: training,
        init_learning_curve: 0.05,
        epochs: 10
    };

    perceptron(percetron_input);
}
