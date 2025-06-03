use std::{error::Error, fs::File};
use serde::{Deserialize};

use crate::types::TrainingData;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Num {
    Float(f32),
    Int(u32),
}

#[derive(Debug, Deserialize)]
pub struct Data{
    pub x:Vec<Vec<Num>>,
    pub y:Vec<Num>
}

pub fn data_deserialized() -> Result<TrainingData, Box<dyn Error>> {
    let file = File::open("data.json")?;
    let data: Data = serde_json::from_reader(file)?; 

    let x:Vec<Vec<f32>> = data.x.into_iter().map(|row| {
        row.into_iter().map(|num|{
            match num {
                Num::Float(f) => f,
                Num::Int(i) => i as f32,
            }
        }).collect()
    }).collect();

    let y = data.y.into_iter().map(|value|{
        match value {
            Num::Float(f) => f,
            Num::Int(i) => i as f32
        }
    }).collect();


     let training_data = TrainingData  {
        x: x,
        y: y
    };

    Ok(training_data)
}