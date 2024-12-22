use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Exhibit;
use log::error;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

pub fn create_exhibit(exhibit: &Exhibit, db_conn: &DbConnection) -> SqliteResult<i64> {
    db_conn.0.execute(
            "INSERT INTO exhibits (name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                exhibit.name,
                exhibit.cluster,
                exhibit.location,
                exhibit.status,
                exhibit.image_url,
                exhibit.sponsor_name,
                exhibit.sponsor_start_date,
                exhibit.sponsor_end_date,
            ],
        )?;
    let exhibit_id = db_conn.0.last_insert_rowid();

    // Associate parts with the exhibit
    for part_id in &exhibit.part_ids {
        db_conn.0.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            params![exhibit_id, part_id],
        )?;
    }

    // Insert notes related to the exhibit
    for note in &exhibit.notes {
        db_conn.0.execute(
            "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
            params![exhibit_id, &note.timestamp, &note.note],
        )?;
    }

    Ok(exhibit_id)
}

/// Handles the POST /exhibits endpoint
#[post("/exhibits", format = "json", data = "<new_exhibit>")]
pub async fn create_exhibit_handler(
    new_exhibit: Json<Exhibit>,
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<i64>, ApiError> {
    let db_conn = db.lock().await;

    match create_exhibit(&new_exhibit.into_inner(), &*db_conn) {
        Ok(id) => Ok(Json(id)),
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::DatabaseError("Database Error".to_string()))
        }
    }
}
