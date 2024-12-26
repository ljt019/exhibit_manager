use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Part;
use log::error;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Creates a new part in the database.
///
/// This function inserts a new part into the `parts` table, associates it with exhibits,
/// and adds any related notes.
///
/// # Arguments
/// * `part` - A reference to the `Part` struct containing the part data.
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<i64>` - The ID of the newly created part.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn create_part(part: &Part, conn: &Connection) -> rusqlite::Result<i64> {
    // Insert the new part
    conn.execute(
        "INSERT INTO parts (name, link) VALUES (?1, ?2)",
        rusqlite::params![part.name, part.link],
    )?;
    let part_id = conn.last_insert_rowid();

    // Associate exhibits with the part
    for exhibit_id in &part.exhibit_ids {
        conn.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            rusqlite::params![exhibit_id, part_id],
        )?;
    }

    // Insert notes related to the part
    for note in &part.notes {
        conn.execute(
            "INSERT INTO part_notes (part_id, date, time, message) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                part_id,
                &note.timestamp.date,
                &note.timestamp.time,
                &note.message
            ],
        )?;
    }

    Ok(part_id)
}

/// Handles the POST /parts endpoint.
///
/// This endpoint creates a new part with associated exhibits and notes.
/// It processes the incoming JSON payload and returns the ID of the newly created part.
///
/// # Arguments
/// * `new_part` - JSON payload containing the part data.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<i64>, ApiError>` - The ID of the newly created part.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - A database operation fails.
#[post("/parts", format = "json", data = "<new_part>")]
pub async fn create_part_handler(
    new_part: Json<Part>,
    db_pool: &State<DbPool>,
) -> Result<Json<i64>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        create_part(&new_part.into_inner(), &conn).map_err(|e| {
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
