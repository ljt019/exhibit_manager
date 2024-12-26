use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Timestamp;
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
/// * `part_id` - The ID of the associated part
/// * `conn` - A reference to the database connection
///
/// # Returns
/// * `rusqlite::Result<i64>` - The ID of the newly created note or a rusqlite error
pub fn create_note(new_note: &NewNote, part_id: i64, conn: &Connection) -> rusqlite::Result<i64> {
    let timestamp = Timestamp {
        date: chrono::Local::now().naive_local().date().to_string(),
        time: chrono::Local::now().naive_local().time().to_string(),
    };

    conn.execute(
        "INSERT INTO part_notes (part_id, date, time, message) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![part_id, &timestamp.date, &timestamp.time, &new_note.message],
    )?;

    let note_id = conn.last_insert_rowid();

    Ok(note_id)
}

/// Creates a new note with associated part.
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
#[post("/parts/<id>/notes", format = "json", data = "<new_note>")]
pub async fn create_part_note_handler(
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
