#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RotationDescription
{
    rotation_name:String,
    responsibilities:Vec<RotationResponsibility>
}