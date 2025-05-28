//funzione decisionale
pub struct PredictionInput{
    pub error:f32,
    pub bias:f32,
    pub total_error:f32,
    pub weight:f32,
    pub learning_curve:f32,
    pub x_data:f32
}

pub struct PredictionOutput{
    pub next_weight:f32,
    pub next_bias:f32,
    pub next_total_error:f32,
}

pub fn prediction_formula (input: PredictionInput) -> Option<PredictionOutput> {
    let PredictionInput { 
        error, 
        bias, 
        total_error,
        weight,
        learning_curve,
        x_data
    } = input;

    if error != 0.0 {
        Some(PredictionOutput {
            next_weight : weight + learning_curve + error + x_data,
            next_bias : bias + learning_curve + error,
            next_total_error : total_error + 1.0
        })
    }
    else {
        None
    }
}    