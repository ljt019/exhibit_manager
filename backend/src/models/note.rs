use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Validate, Clone, FromRow)]
pub struct Timestamp {
    #[validate(custom(function = "Timestamp::validate_date_format"))]
    pub date: String,
    #[validate(custom(function = "Timestamp::validate_date_format"))]
    pub time: String,
}

impl Timestamp {
    fn validate_date_format(date: &str) -> Result<(), ValidationError> {
        chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| ValidationError::new("Date must be in YYYY-MM-DD format"))?;
        Ok(())
    }
}

// Also need to validate the Note struct
#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone, FromRow)]
pub struct Note {
    pub id: i64,

    pub submitter: String,

    #[validate(length(min = 1, message = "Note cannot be empty"))]
    pub message: String,

    #[sqlx(flatten)]
    pub timestamp: Timestamp,
}
