pub const ZERO:f32 = 0 as f32;
pub const ONE:f32 = 1 as f32;

pub fn populate_weights(y:usize) -> Vec<f32> {
    let mut start_weights:Vec<f32>= Vec::new();
    for _ in 0..y {
        start_weights.push(ZERO);
    }
    return start_weights
}

