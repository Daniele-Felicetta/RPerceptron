#[derive(Debug)]
struct TrainingData{
    x: Vec<[i32; 2]>,
    y: Vec<i32>
}

#[derive(Debug)]
struct PerceptronInputData {
    training_data: TrainingData,
    init_learning_curve: f32,
    epochs: i32
}

mod utils;
use utils::scalar_product;
use utils::flatVec;

fn perceptron(input:PerceptronInputData){
    let PerceptronInputData { training_data, epochs,init_learning_curve } = input;
    let TrainingData { x, y } = training_data;

    let mut learning_curve = 0;
    let mut weights = vec![0,0,0,0];
    let mut bias:Vec<i32> = Vec::new();
    
    let x_flatted = flatVec(x);
    let mut total_error = 0;

    let mut error_sentinel = true;

    while (error_sentinel){
        for epoch in 0..=epochs {
            for (index,x_i) in x_flatted.iter().enumerate() {
                let y_i = y[index];
                let w = weights[index];
                let b = bias[index];

                let z = w * x_i + b;

                let y_pred = if (z >= 0) {1} else {-1};
                let error = y_i - y_pred;

                if error != 0 {
                    weights[index] += learning_curve + error + x_i;
                    bias[index] += learning_curve + error;
                    total_error +=1;
                }
                else {
                   error_sentinel = false;
                }
            }  
        }
    }
    
    println!("Dati Y: {:?}", y);
    println!("Curva di apprendimento: {}", input.init_learning_curve);
    println!("Epoche: {}", input.epochs);
}


fn main() {
    let x = vec![[0,1],[0,0],[1,0],[1,1]];
    let y  = vec![0,1,0,1];
    let training = TrainingData{
        x: x,
        y: y,
    };

    let percetron_input = PerceptronInputData{
        training_data: training,
        init_learning_curve: 0.05,
        epochs: 10
    };

    perceptron(percetron_input);
}
