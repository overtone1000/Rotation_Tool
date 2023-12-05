use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RotationBaseline
{
    pub rotation:String,
    pub RVU:f64,
    pub BVU:f64
}