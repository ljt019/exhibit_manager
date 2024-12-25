use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rusqlite::Connection;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NewNote {
    #[validate(length(min = 1, message = "Note cannot be empty"))]
    pub message: String,
}

/// Inserts a new note into the exhibit_notes table and returns its ID.
///
/// # Arguments
/// * `note` - A reference to the Note to be inserted
/// * `exhibit_id` - The ID of the associated exhibit
/// * `conn` - A reference to the database connection
///
/// # Returns
/// * `rusqlite::Result<i64>` - The ID of the newly created note or a rusqlite error
pub fn create_note(
    new_note: &NewNote,
    exhibit_id: i64,
    conn: &Connection,
) -> rusqlite::Result<i64> {
    let timestamp = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO exhibit_notes (exhibit_id, timestamp, message) VALUES (?1, ?2, ?3)",
        rusqlite::params![exhibit_id, &timestamp, &new_note.message],
    )?;

    let note_id = conn.last_insert_rowid();

    Ok(note_id)
}

/// Creates a new note with associated exhibit.
///
/// # Arguments
/// * `new_note` - JSON payload containing the note data
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Json<i64>, ApiError>` - The ID of the newly created note
///
/// # Errors
/// Returns `ApiError` if:
/// * Database operations fail
/// * Input validation fails
#[post("/exhibits/<id>/notes", format = "json", data = "<new_note>")]
pub async fn create_exhibit_note_handler(
    id: i64,
    new_note: Json<NewNote>,
    db_pool: &State<DbPool>,
) -> Result<Json<i64>, ApiError> {
    let note = new_note.into_inner();
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().expect("Failed to get DB connection from pool");
        create_note(&note, id, &conn)
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(result))
}
