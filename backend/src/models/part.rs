use crate::models::note::Note;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Part {
    pub id: Option<i64>,
    pub name: String,
    pub link: String,
    pub exhibit_ids: Vec<i64>,
    pub notes: Vec<Note>,
}
