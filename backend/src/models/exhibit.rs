use crate::models::note::Note;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct Sponsor {
    pub name: String,

    #[validate(custom(function = "Sponsor::validate_date_format"))]
    pub start_date: String,

    #[validate(custom(function = "Sponsor::validate_date_format"))]
    pub end_date: String,
}

impl Sponsor {
    fn validate_date_format(date: &str) -> Result<(), ValidationError> {
        chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| ValidationError::new("Date must be in YYYY-MM-DD format"))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct Exhibit {
    pub id: i64,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Cluster must be between 1 and 100 characters"
    ))]
    pub cluster: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Location must be between 1 and 100 characters"
    ))]
    pub location: String,

    pub description: String,

    #[validate(custom(function = "Exhibit::validate_status"))]
    pub status: String,

    pub part_ids: Vec<i64>,

    // This will validate each Note in the Vec with the notes struct validation
    #[validate(nested)]
    pub notes: Vec<Note>,

    #[validate(url)]
    pub image_url: String,

    pub sponsor: Option<Sponsor>,
}

impl Exhibit {
    fn validate_status(status: &str) -> Result<(), ValidationError> {
        match status.to_lowercase().as_str() {
            "active" | "inactive" | "maintenance" => Ok(()),
            _ => Err(ValidationError::new(
                "Invalid status. Must be 'active', 'inactive', or 'maintenance'",
            )),
        }
    }
}
