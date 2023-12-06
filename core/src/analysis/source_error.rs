use std::fmt;

pub struct SourceError {
    message: String,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source error: {}", self.message) // user-facing output
    }
}

impl fmt::Debug for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
        write!(f, "Source error: {}", self.message) // user-facing output
    }
}

impl std::error::Error for SourceError {}

impl SourceError {
    pub fn generate<T>(message: String) -> Result<T, SourceError> {
        Err(SourceError { message })
    }

    pub fn generate_boxed<T>(message: String) -> Result<T, Box<dyn std::error::Error>> {
        Err(Box::new(SourceError { message }))
    }
}
