mod perceptron;
mod types;
mod utils;
mod formulas;

use types::PerceptronInputData;
use types::TrainingData;
use perceptron::perceptron;
use utils::populate_weights;
use utils::{ONE,ZERO};


fn main() {
    let training = TrainingData{
        x: vec![[ZERO,ONE],[ZERO,ZERO],[ONE,ZERO],[ONE,ONE]],
        y:vec![ZERO,ZERO,ZERO,ONE],
    };
    let y_length = training.y.len();

    let percetron_input = PerceptronInputData{
        training_data: training,
        init_learning_curve: 0.05,
        epochs: 10,
        init_weights: vec![ZERO,ZERO,ZERO,ZERO]
    };

    populate_weights(y_length);

    perceptron(percetron_input);
}

  