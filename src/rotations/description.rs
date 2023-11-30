use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use super::responsibility::RotationResponsibility;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RotationDescription
{
    pub(crate) rotation:String,
    pub(crate) location:String,
    pub(crate) responsibilities:Option<Vec<RotationResponsibility>>,
    pub(crate) comments:Option<HashSet<String>>
}