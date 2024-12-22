use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Part;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

fn update_part(id: i64, part: &Part, db_conn: &DbConnection) -> SqliteResult<usize> {
    let affected = db_conn.0.execute(
        "UPDATE parts SET name = ?1, link = ?2 WHERE id = ?3",
        params![part.name, part.link, id],
    )?;

    // Update associated exhibits: Remove existing and add new associations
    db_conn
        .0
        .execute("DELETE FROM exhibit_parts WHERE part_id = ?1", params![id])?;
    for exhibit_id in &part.exhibit_ids {
        db_conn.0.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            params![exhibit_id, id],
        )?;
    }

    // Update notes: Remove existing and add new notes
    db_conn
        .0
        .execute("DELETE FROM part_notes WHERE part_id = ?1", params![id])?;
    for note in &part.notes {
        db_conn.0.execute(
            "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
            params![id, &note.timestamp, &note.note],
        )?;
    }

    Ok(affected)
}

/// Handles the PUT /parts/<id> endpoint
#[put("/parts/<id>", format = "json", data = "<updated_part>")]
pub async fn update_part_handler(
    id: i64,
    updated_part: Json<Part>,
    db: &State<Mutex<DbConnection>>,
) -> Result<Status, ApiError> {
    let db_conn = db.lock().await;

    match update_part(id, &updated_part.into_inner(), &*db_conn) {
        Ok(updated) if updated > 0 => Ok(Status::Ok),
        Ok(_) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
