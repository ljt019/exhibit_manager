use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// Also need to validate the Note struct
#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct Note {
    #[validate(length(min = 1, message = "Note cannot be empty"))]
    pub note: String,

    #[validate(custom(function = "validate_timestamp"))]
    pub timestamp: String,
}

fn validate_timestamp(timestamp: &str) -> Result<(), ValidationError> {
    // Validate timestamp format (assuming ISO 8601)
    // You might want to use chrono here for proper date validation
    if timestamp.len() < 1 {
        return Err(ValidationError::new("Timestamp cannot be empty"));
    }
    Ok(())
}
