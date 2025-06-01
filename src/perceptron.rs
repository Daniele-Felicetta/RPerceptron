use crate::utils::{scalar_product};
use crate::types::PerceptronInputData;
use crate::types::TrainingData;
use crate::formulas::{prediction_formula, PredictionInput};

pub fn perceptron(input:PerceptronInputData){
    let PerceptronInputData { 
        training_data, 
        epochs,
        init_learning_curve,
        init_weights 
    } = input;

    let TrainingData { x, y } = training_data;

    let mut weights= init_weights;
    let learning_curve = init_learning_curve;
    let mut bias = 0.0;
    
    let mut total_error = 0.0;

    for _epoch in 0..=epochs {
        for (index,x_i) in x.iter().enumerate() {
            let y_i = y[index];
            let x_i_vec= x_i.to_vec();
            let z = scalar_product(&weights, &x_i_vec) + bias;

            let y_pred = if z >= 0.0 {1} else {-1};

            let prediction_input = PredictionInput{
                weights: &mut weights,
                error:y_i - y_pred as f32,
                total_error:& mut total_error,
                bias:&mut bias,
                learning_curve:learning_curve,
                x_data:x_i_vec
            };
            prediction_formula(prediction_input);
        }  
    }
}