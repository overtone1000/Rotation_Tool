use serde::{Serialize, Deserialize};

use super::responsibility::RotationResponsibility;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationDescription
{
    pub(crate) rotation:String,
    pub(crate) location:String,
    pub(crate) responsibilities:Option<Vec<RotationResponsibility>>,
}