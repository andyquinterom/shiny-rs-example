use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AreaSelect {
    pub x: [i32; 2],
    pub y: [i32; 2],
}
