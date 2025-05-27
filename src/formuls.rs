//funzione decisionale
struct PredictionInput{
    error:i32,
    bias:i32,
    total_error:i32
}

struct PredictionOutput{
    bias:i32,
    total_error:i32,
}

fn prediction (input: PredictionInputStruct) -> PredictionOutput {
    let PredictionInput { error, bias, total_error} = input;

    if error != 0 {
        weights[index] += learning_curve + error + x_i;
        bias += learning_curve + error;
        total_error +=1;
    }
    else {
        error_sentinel = false;
    }

    return {
        error_sentinel
    }
}    