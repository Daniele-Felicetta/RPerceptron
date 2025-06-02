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

    println!("--- ALLENAMENTO COMPLETATO ---");
    println!("Pesi finali: {:?}", weights);
    println!("Bias finale: {}", bias);

    println!("--- PREVISIONI FINALI ---");
    for (index, x_i) in x.iter().enumerate() {
        let z = scalar_product(&weights, &x_i.to_vec()) + bias;
        let y_pred = if z >= 0.0 { 1 } else { -1 };
        println!("Esempio {}: input = {:?}, predizione = {}, atteso = {}", 
                index, x_i, y_pred, y[index]);
    }

    let mut correct = 0;
    for (index, x_i) in x.iter().enumerate() {
        let z = scalar_product(&weights, &x_i.to_vec()) + bias;
        let y_pred = if z >= 0.0 { 1.0 } else { -1.0 };  // y è f32
        if y_pred == y[index] {
            correct += 1;
        }
    }
    let accuracy = (correct as f32 / y.len() as f32) * 100.0;
    println!("Accuracy finale: {:.2}%", accuracy);
}