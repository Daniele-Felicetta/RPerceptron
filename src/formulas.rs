//funzione decisionale
pub struct PredictionInput{
    pub error:f32,
    pub bias:f32,
    pub total_error:f32,
    pub weights:&mut[f32],
    pub learning_curve:f32,
    pub x_data:Vec<f32>
}

pub struct PredictionOutput{
    pub new_weights:f32,
    pub next_bias:f32,
    pub next_total_error:f32,
}

pub fn prediction_formula (input: PredictionInput) -> Option<PredictionOutput> {
    let PredictionInput { 
        error, 
        bias, 
        total_error,
        weights,
        learning_curve,
        x_data
    } = input;

    let mut new_weights =
    if error != 0.0 {
         for (index, value) in x_data.iter().enumerate(){
             = weights[index] + learning_curve       
        }
        Some(PredictionOutput {
           
            next_weights : weights + learning_curve * error * x_data,
            next_bias : bias + learning_curve * error,
            next_total_error : total_error + 1.0
        })
    }
    else {
        None
    }
}    