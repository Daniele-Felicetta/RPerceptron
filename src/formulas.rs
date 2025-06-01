//funzione decisionale
pub struct PredictionInput<'a>{
    pub error:f32,
    pub x_data:Vec<f32>,
    pub learning_curve: f32,
    pub bias:&'a mut f32,
    pub total_error:&'a mut f32,
    pub weights:&'a mut[f32]
}

pub fn prediction_formula (input: PredictionInput) {
    let PredictionInput { 
        error, 
        bias, 
        total_error,
        weights,
        learning_curve,
        x_data
    } = input;

    if error != 0.0 {
        for (index, value) in x_data.iter().enumerate(){
            weights[index] = weights[index] + learning_curve *error * value;
        }
        *bias += learning_curve * error;
        *total_error += 1.0; 
    }
}    