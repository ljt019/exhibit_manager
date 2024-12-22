use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Exhibit;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

pub fn update_exhibit(id: i64, exhibit: &Exhibit, db_conn: &DbConnection) -> SqliteResult<usize> {
    let affected = db_conn.0.execute(
        "UPDATE exhibits 
             SET name = ?1, cluster = ?2, location = ?3, status = ?4, image_url = ?5, 
                 sponsor_name = ?6, sponsor_start_date = ?7, sponsor_end_date = ?8 
             WHERE id = ?9",
        params![
            exhibit.name,
            exhibit.cluster,
            exhibit.location,
            exhibit.status,
            exhibit.image_url,
            exhibit.sponsor_name,
            exhibit.sponsor_start_date,
            exhibit.sponsor_end_date,
            id
        ],
    )?;

    // Update associated parts: Remove existing and add new associations
    db_conn.0.execute(
        "DELETE FROM exhibit_parts WHERE exhibit_id = ?1",
        params![id],
    )?;
    for part_id in &exhibit.part_ids {
        db_conn.0.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            params![id, part_id],
        )?;
    }

    // Update notes: Remove existing and add new notes
    db_conn.0.execute(
        "DELETE FROM exhibit_notes WHERE exhibit_id = ?1",
        params![id],
    )?;
    for note in &exhibit.notes {
        db_conn.0.execute(
            "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
            params![id, &note.timestamp, &note.note],
        )?;
    }

    Ok(affected)
}

/// Handles the PUT /exhibits/<id> endpoint
#[put("/exhibits/<id>", format = "json", data = "<updated_exhibit>")]
pub async fn update_exhibit_handler(
    id: i64,
    updated_exhibit: Json<Exhibit>,
    db: &State<Mutex<DbConnection>>,
) -> Result<Status, ApiError> {
    let db_conn = db.lock().await;

    match update_exhibit(id, &updated_exhibit.into_inner(), &*db_conn) {
        Ok(updated) if updated > 0 => Ok(Status::Ok),
        Ok(_) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
