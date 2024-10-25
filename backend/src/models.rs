use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Exhibit {
    pub id: Option<i64>,
    pub name: String,
    pub cluster: String,
    pub location: String,
    pub status: String,
    pub part_ids: Vec<i64>,
    pub notes: Vec<Note>,
    pub image_url: String,
    pub sponsor_name: Option<String>,
    pub sponsor_start_date: Option<String>,
    pub sponsor_end_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub id: Option<i64>,
    pub name: String,
    pub link: String,
    pub exhibit_ids: Vec<i64>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub timestamp: String,
    pub note: String,
}
