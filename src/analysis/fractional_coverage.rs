#[derive(Debug,PartialEq)]
pub struct FractionalCoverageUnit
{
    rotation:String,
    fraction:f64
}

impl FractionalCoverageUnit
{
    pub fn create(rotation:String, fraction:f64)->FractionalCoverageUnit
    {
        FractionalCoverageUnit
        {
            rotation: rotation,
            fraction: fraction
        }
    }

    pub fn get_fraction(&self)->f64{self.fraction}
}