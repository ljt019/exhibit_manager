use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmissionDate {
    pub date: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Validate)]
pub struct Jotform {
    pub id: String,
    pub submitter_name: String,
    pub created_at: SubmissionDate,
    pub location: String,
    pub exhibit_name: String,
    pub description: String,

    #[validate(custom(function = "validate_priority_level"))]
    pub priority_level: String,

    #[validate(custom(function = "validate_department"))]
    pub department: String,

    #[validate(custom(function = "validate_status"))]
    pub status: String,
}

fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "Open" => Ok(()),
        "InProgress" => Ok(()),
        "Closed" => Ok(()),
        _ => Err(ValidationError::new("Invalid status")),
    }
}

fn validate_department(department: &str) -> Result<(), ValidationError> {
    match department {
        "Exhibits" => Ok(()),
        "Operations" => Ok(()),
        _ => Err(ValidationError::new("Invalid department")),
    }
}

fn validate_priority_level(priority_level: &str) -> Result<(), ValidationError> {
    match priority_level {
        "Low" => Ok(()),
        "Medium" => Ok(()),
        "High" => Ok(()),
        _ => Err(ValidationError::new("Invalid priority level")),
    }
}
