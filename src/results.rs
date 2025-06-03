use crate::{formulas::scalar_product};

pub fn results(weights:Vec<f32>, bias:f32,x:Vec<Vec<f32>>, y:Vec<f32>, epochs:i32, learning_curve:f32){    
    println!("--- ALLENAMENTO COMPLETATO ---");
    println!("Epoche: {}", epochs);
    println!("Learning curve: {}", learning_curve);
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