//funzione decisionale
pub struct PredictionInput{
    pub error:i32,
    pub bias:i32,
    pub total_error:i32,
    pub weight:i32,
    pub learning_curve:i32,
    pub x_data:i32
}

pub struct PredictionOutput{
    pub next_weight:i32,
    pub next_bias:i32,
    pub next_total_error:i32,
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

    if error != 0 {
        Some(PredictionOutput {
            next_weight : weight + learning_curve + error + x_data,
            next_bias : bias + learning_curve + error,
            next_total_error : total_error + 1
        })
    }
    else {
        None
    }
}    