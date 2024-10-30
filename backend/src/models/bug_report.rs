use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BugReport {
    pub name: String,
    pub title: String,
    pub description: String,
}
