use serde::{Serialize, Deserialize};

use super::responsibility::RotationResponsibility;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationDescription
{
    pub(crate) rotation_name:String,
    pub(crate) responsibilities:Vec<RotationResponsibility>
}