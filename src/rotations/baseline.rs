use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationBaseline
{
    rotation_name:String,
}