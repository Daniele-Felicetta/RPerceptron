mod perceptron;
mod types;
mod utils;
mod formulas;
mod results; 
mod deserialize;

use types::PerceptronInputData;
use perceptron::perceptron;
use utils::populate_weights;
use deserialize::data_deserialized;

fn main() {
    let training_data = data_deserialized().unwrap();

    let y_length = training_data.x[0].len();

    let init_weigths=populate_weights(y_length);

    let percetron_input = PerceptronInputData{
        training_data: training_data,
        init_learning_curve: 0.1,
        epochs: 100000,
        init_weights: init_weigths,
        view_results:true
    };

    perceptron(percetron_input);
}

  