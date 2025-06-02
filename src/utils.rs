pub const ZERO:f32 = 0 as f32;
pub const ONE:f32 = 1 as f32;

pub fn scalar_product(w:&[f32], x:&[f32]) -> f32{
    let mut result = ZERO;
    for (index,w_i) in w.iter().enumerate() {
        let x_i = x[index];
        result = result + x_i * w_i;
    }  

    return result;
}


pub fn flat_vec(matrix:Vec<[f32;2]>) -> Vec<f32> {
    let mut flat = Vec::new();

    for row in matrix {
        for val in row {
            flat.push(val)
        }
    }
    return flat;
} 


pub fn populate_weights(y:usize) -> Vec<f32> {
    let mut start_weights:Vec<f32>= Vec::new();
    for _ in 0..y {
        start_weights.push(ZERO);
    }
    return start_weights
}