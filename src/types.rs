#[derive(Debug)]
pub struct TrainingData{
    pub x: Vec<[i32; 2]>,
    pub y: Vec<i32>
}

#[derive(Debug)]
pub struct PerceptronInputData {
    pub training_data: TrainingData,
    pub init_learning_curve: f32,
    pub epochs: i32
}
