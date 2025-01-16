use crate::models::{Note, Sponsor};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct UpdateExhibit {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Cluster must be between 1 and 100 characters"
    ))]
    pub cluster: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Location must be between 1 and 100 characters"
    ))]
    pub location: Option<String>,

    pub description: Option<String>,

    #[validate(url)]
    pub image_url: Option<String>,
}

impl UpdateExhibit {
    fn validate_status(status: &str) -> Result<(), ValidationError> {
        match status.to_lowercase().as_str() {
            "active" | "inactive" | "maintenance" => Ok(()),
            _ => Err(ValidationError::new(
                "Invalid status. Must be 'active', 'inactive', or 'maintenance'",
            )),
        }
    }
}
