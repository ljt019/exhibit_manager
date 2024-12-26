use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Note;
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

pub fn list_exhibit_notes(exhibit_id: i64, conn: &Connection) -> rusqlite::Result<Vec<Note>> {
    let mut stmt =
        conn.prepare("SELECT id, timestamp, message FROM exhibit_notes WHERE exhibit_id = ?1")?;
    let notes_iter = stmt.query_map([exhibit_id], |row| {
        Ok(Note {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            message: row.get(2)?,
        })
    })?;

    let mut notes = Vec::new();
    for note_res in notes_iter {
        let note = note_res?;
        notes.push(note);
    }

    Ok(notes)
}

#[get("/exhibits/<exhibit_id>/notes")]
pub async fn list_exhibit_notes_handler(
    exhibit_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Note>>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        list_exhibit_notes(exhibit_id, &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(result))
}
