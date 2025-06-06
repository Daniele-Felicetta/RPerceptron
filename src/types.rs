#[derive(Debug)]
pub struct TrainingData {
    pub x: Vec<Vec<f32>>,  // Non più array fisso!
    pub y: Vec<f32>,
}
#[derive(Debug)]
pub struct PerceptronInputData {
    pub training_data: TrainingData,
    pub init_learning_curve: f32,
    pub epochs: i32,
    pub init_weights: Vec<f32>,
    pub view_results: bool
}
