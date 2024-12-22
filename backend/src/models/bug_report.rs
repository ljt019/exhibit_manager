use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, PartialEq, Eq, Validate)]
pub struct BugReport {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Title must be between 1 and 100 characters"
    ))]
    pub title: String,

    #[validate(length(
        min = 1,
        max = 250,
        message = "Description must be between 1 and 250 characters"
    ))]
    pub description: String,
}
