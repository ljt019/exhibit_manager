use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Note, Part};
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Retrieves all parts from the database.
///
/// This function fetches all parts along with their associated exhibit IDs and notes.
///
/// # Arguments
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<Vec<Part>>` - A vector of parts.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn list_parts(conn: &Connection) -> rusqlite::Result<Vec<Part>> {
    let mut stmt = conn.prepare("SELECT id, name, link FROM parts")?;
    let parts_iter = stmt.query_map([], |row| {
        Ok(Part {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            link: row.get(2)?,
            exhibit_ids: Vec::new(),
            notes: Vec::new(),
        })
    })?;

    let mut parts = Vec::new();
    for part_res in parts_iter {
        let mut part = part_res?;
        let id = part.id.unwrap();

        // Fetch associated exhibit IDs
        let mut stmt_exhibits =
            conn.prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
        let exhibit_ids_iter = stmt_exhibits.query_map(rusqlite::params![id], |row| row.get(0))?;
        part.exhibit_ids = exhibit_ids_iter.collect::<rusqlite::Result<Vec<i64>>>()?;

        // Fetch associated notes
        let mut stmt_notes =
            conn.prepare("SELECT id, timestamp, message FROM part_notes WHERE part_id = ?1")?;
        let notes_iter = stmt_notes.query_map(rusqlite::params![id], |row| {
            Ok(Note {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                message: row.get(2)?,
            })
        })?;
        part.notes = notes_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

        parts.push(part);
    }

    Ok(parts)
}

/// Handles the GET /parts endpoint.
///
/// This endpoint retrieves a list of all parts from the database.
///
/// # Arguments
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Part>>, ApiError>` - A JSON array of parts.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - A database operation fails.
#[get("/parts")]
pub async fn list_parts_handler(db_pool: &State<DbPool>) -> Result<Json<Vec<Part>>, ApiError> {
    let pool = (*db_pool).clone();

    let parts_result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        list_parts(&conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(parts_result))
}
