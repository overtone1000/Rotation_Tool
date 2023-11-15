use serde::{Serialize, Deserialize};

use super::stringtypes::StringTypes;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationResponsibility
{
    pub sites:StringTypes,
    pub subspecialties:StringTypes,
    pub contexts:StringTypes,
    pub modalities:StringTypes,
    pub days:StringTypes,
    pub time_periods:StringTypes,
}