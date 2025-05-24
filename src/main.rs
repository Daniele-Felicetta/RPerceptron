#[derive(Debug)]
struct TrainingData{
    x: Vec<[i32; 2]>,
    y: Vec<i32>
}

#[derive(Debug)]
struct PerceptronInputData {
    training_data: TrainingData,
    learning_curve: f32,
    epochs: i32
}


fn perceptron(input:PerceptronInputData){
    let mut weights = vec![0,0,0,0];
    let mut bias = 0;

    for epoch in 0..=input.epochs {
        let mut total_error = 0;
        let PerceptronInputData { training_data, epochs,learning_curve } = input;
        let TrainingData { x, y } = training_data;

        for (index,x_i) in x.iter().enumerate() {
            let y_i = y[index];

            let z = weights * x_i + bias;
             
        }  
    }

    println!("Dati X: {:?}",input.training_data.x);
    println!("Dati Y: {:?}",input.training_data.y);
    println!("Curva di apprendimento: {}", input.learning_curve);
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
        learning_curve: 0.05,
        epochs: 10
    };

    perceptron(percetron_input);
}
