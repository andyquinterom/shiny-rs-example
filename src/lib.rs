use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SimpleStruct {
    pub x: f64,
    pub y: f64,
    pub name: String,
}
