use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Note, Timestamp};
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

pub fn get_exhibit_note(
    exhibit_id: i64,
    note_id: i64,
    conn: &Connection,
) -> rusqlite::Result<Option<Note>> {
    let note_opt = conn
        .query_row(
            "SELECT id, date, time, message FROM exhibit_notes WHERE exhibit_id = ?1 AND id = ?2",
            rusqlite::params![exhibit_id, note_id],
            |row| {
                let timestamp = Timestamp {
                    date: row.get(1)?,
                    time: row.get(2)?,
                };

                Ok(Note {
                    id: row.get(0)?,
                    timestamp,
                    message: row.get(3)?,
                })
            },
        )
        .optional()?;

    Ok(note_opt)
}

#[get("/exhibits/<exhibit_id>/notes/<note_id>")]
pub async fn get_exhibit_note_handler(
    exhibit_id: i64,
    note_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Note>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        get_exhibit_note(exhibit_id, note_id, &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    match result {
        Some(note) => Ok(Json(note)),
        None => Err(ApiError::NotFound),
    }
}
