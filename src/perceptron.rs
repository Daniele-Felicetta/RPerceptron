use crate::utils::{flat_vec};
use crate::types::PerceptronInputData;
use crate::types::TrainingData;
use crate::formulas::{prediction_formula, PredictionInput, PredictionOutput};


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
    
    let x_flatted = flat_vec(x);
    let mut total_error = 0.0;

    let mut error_sentinel = true;


    while error_sentinel {
        for _epoch in 0..=epochs {
            for (index,x_i) in x_flatted.iter().enumerate() {
                let y_i = y[index];
                let weight:f32 = weights[index];
                let z = weight * x_i + bias;

                let y_pred = if z >= 0.0 {1} else {-1};

                let prediction_input = PredictionInput{
                    weight: weight,
                    error:y_i - y_pred as f32,
                    total_error:total_error,
                    bias:bias,
                    learning_curve:learning_curve,
                    x_data:*x_i
                };

                let output_prediction = prediction_formula(prediction_input);

                match output_prediction {
                    None => {
                        error_sentinel=false;
                    },
                    Some(PredictionOutput{next_total_error,next_weight,next_bias}) =>{
                        total_error =next_total_error;
                        weights[index] =next_weight;
                        bias= next_bias;
                    }
                }
            }  
        }
    }
    
    // println!("Dati Y: {:?}", y);
    // println!("Curva di apprendimento: {}", input.init_learning_curve);
    // println!("Epoche: {}", input.epochs);
}