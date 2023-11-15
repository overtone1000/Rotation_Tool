use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationBaseline
{
    pub rotation:String,
    pub RVU:f64,
    pub BVU:f64
}