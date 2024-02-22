use crate::rotations::time_modifiers::TimeSinceMidnight;


#[derive(Default, Debug)]
pub struct MalformedCoverage {
    pub gaps: Vec<(TimeSinceMidnight, TimeSinceMidnight, String, f64)>,
    pub overlaps: Vec<String>,
    pub incorrect_fraction: Option<f64>,
    pub no_work: bool
}

pub enum CoverageError {
    NoCoverage(f64),
    MalformedCoverage(MalformedCoverage),
}