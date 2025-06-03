use crate::types::PerceptronInputData;
use crate::types::TrainingData;
use crate::formulas::{prediction_formula, PredictionInput, scalar_product};
use crate::results::{results};


pub fn perceptron(input:PerceptronInputData){
    let PerceptronInputData { 
        training_data, 
        epochs,
        init_learning_curve,
        init_weights,
        view_results
    } = input;

    let TrainingData { x, y } = training_data;

    let mut weights= init_weights;
    let learning_curve = init_learning_curve;
    let mut bias = 0.0;
    
    let mut total_error = 0.0;

    for _epoch in 0..=epochs {
        for (x_i,y_i) in x.iter().zip(y.iter()) {
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

    if view_results {
        results(weights, bias, x, y, epochs, learning_curve);
    }
}