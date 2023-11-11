#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationResponsibility
{
    site:StringTypes,
    subspecialty:StringTypes,
    context:StringTypes,
    modality:StringTypes,
    time_period:StringTypes,
    day:String
}