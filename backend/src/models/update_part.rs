use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct UpdatePart {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(url)]
    pub link: String,

    #[serde(rename = "exhibitIds")]
    pub exhibit_ids: Vec<i64>,
}
