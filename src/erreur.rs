use std::fmt;

#[derive(Debug)]
pub struct InvalidCoordinatesError;

impl fmt::Display for InvalidCoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid coordinates")
    }
}
impl std::error::Error for InvalidCoordinatesError {}
