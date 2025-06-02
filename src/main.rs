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
        x: vec![
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 1.0],
        ],
        y: vec![
            -1.0,  
            1.0, 
            1.0,  
            1.0, 
        ]
    };
    let y_length = training.x[0].len();

    let init_weigths=populate_weights(y_length);

    let percetron_input = PerceptronInputData{
        training_data: training,
        init_learning_curve: 0.1,
        epochs: 10,
        init_weights: init_weigths
    };

    perceptron(percetron_input);
}

  