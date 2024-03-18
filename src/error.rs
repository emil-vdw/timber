use crate::location::Location;

#[derive(Debug, Clone)]
pub struct ScanError {
    message: String,
    location: Location,
}

impl ScanError {
    pub fn new(message: &str, location: Location) -> Self {
        Self {
            message: message.to_string(),
            location,
        }
    }
}
