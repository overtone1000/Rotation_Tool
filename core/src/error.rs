use std::fmt::{Debug, Display, Formatter, Result};

pub struct RotationToolError {
    message: String,
}

impl std::error::Error for RotationToolError {}

impl Display for RotationToolError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Rotation Tool Error: {}", self.message) // user-facing output
    }
}

impl Debug for RotationToolError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return <RotationToolError as Display>::fmt(self, f);
        //write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl RotationToolError {
    pub fn new(message: String) -> RotationToolError {
        RotationToolError { message: message }
    }
}
