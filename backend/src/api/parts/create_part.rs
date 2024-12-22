use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Part;
use log::error;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

fn create_part(part: &Part, db_conn: &DbConnection) -> SqliteResult<i64> {
    db_conn.access().execute(
        "INSERT INTO parts (name, link) VALUES (?1, ?2)",
        params![part.name, part.link],
    )?;
    let part_id = db_conn.0.last_insert_rowid();

    // Associate exhibits with the part
    for exhibit_id in &part.exhibit_ids {
        db_conn.access().execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            params![exhibit_id, part_id],
        )?;
    }

    // Insert notes related to the part
    for note in &part.notes {
        db_conn.access().execute(
            "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
            params![part_id, &note.timestamp, &note.note],
        )?;
    }

    Ok(part_id)
}

/// Handles the POST /parts endpoint
#[post("/parts", format = "json", data = "<new_part>")]
pub async fn create_part_handler(
    new_part: Json<Part>,
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<i64>, ApiError> {
    let db_conn = db.lock().await;

    match create_part(&new_part.into_inner(), &*db_conn) {
        Ok(id) => Ok(Json(id)),
        Err(e) => {
            error!("Database error: {}", e);
            Err(ApiError::DatabaseError("Database Error".to_string()))
        }
    }
}
