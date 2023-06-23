use std::fmt;

#[derive(Debug)]
pub struct InvalidCoordinatesError;

impl fmt::Display for InvalidCoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid coordinates")
    }
}
impl std::error::Error for InvalidCoordinatesError {}

#[derive(Debug)]
pub struct AlreadyHolding;

impl fmt::Display for AlreadyHolding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hold is already holding a tetro")
    }
}

impl std::error::Error for AlreadyHolding {}

#[derive(Debug)]
pub struct NothingHolded;

impl fmt::Display for NothingHolded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hold is not holding a tetro")
    }
}

impl std::error::Error for NothingHolded {}
