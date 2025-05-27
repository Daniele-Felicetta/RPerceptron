use crate::utils::{flatVec};
use crate::types::PerceptronInputData;
use crate::types::TrainingData;

pub fn perceptron(input:PerceptronInputData){
    let PerceptronInputData { training_data, epochs,init_learning_curve } = input;
    let TrainingData { x, y } = training_data;

    let mut learning_curve = 0;
    let mut weights = vec![0,0,0,0];
    let mut bias = 0;
    
    let x_flatted = flatVec(x);
    let mut total_error = 0;

    let mut error_sentinel = true;


    while error_sentinel {
        for _epoch in 0..=epochs {
            for (index,x_i) in x_flatted.iter().enumerate() {
                let y_i = y[index];
                let w = weights[index];

                let z = w * x_i + bias;

                let y_pred = if z >= 0 {1} else {-1};
                let error = y_i - y_pred;

                prediction(error);
            }  
        }
    }
    
    // println!("Dati Y: {:?}", y);
    // println!("Curva di apprendimento: {}", input.init_learning_curve);
    // println!("Epoche: {}", input.epochs);
}