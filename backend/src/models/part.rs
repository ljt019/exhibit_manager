use crate::models::note::Note;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Validate)]
pub struct Part {
    pub id: Option<i64>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(url)]
    pub link: String,

    pub exhibit_ids: Vec<i64>,

    #[validate(nested)]
    pub notes: Vec<Note>,
}
