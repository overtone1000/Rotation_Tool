use std::fmt;

pub struct RotationManifestParseError {
    line: u64,
    message: String,
}

impl fmt::Display for RotationManifestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rotation manifest parsing error at line {}: {}",
            self.line, self.message
        ) // user-facing output
    }
}

impl fmt::Debug for RotationManifestParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
        write!(
            f,
            "Rotation manifest parsing error at line {}: {}",
            self.line, self.message
        ) // user-facing output
    }
}

impl std::error::Error for RotationManifestParseError {}

impl RotationManifestParseError {
    pub fn generate<T>(line: u64, message: String) -> Result<T, RotationManifestParseError> {
        Err(RotationManifestParseError {
            line: line,
            message: message,
        })
    }

    pub fn generate_boxed<T>(line: u64, message: String) -> Result<T, Box<dyn std::error::Error>> {
        Err(Box::new(
            (RotationManifestParseError {
                line: line,
                message: message,
            }),
        ))
    }
}
