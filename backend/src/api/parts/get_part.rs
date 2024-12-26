use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Note, Part, Timestamp};
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

/// Retrieves a part from the database.
///
/// This function fetches a part by its ID, along with its associated exhibit IDs and notes.
///
/// # Arguments
/// * `id` - The ID of the part to retrieve.
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<Option<Part>>` - The part if found.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn get_part(id: i64, conn: &Connection) -> rusqlite::Result<Option<Part>> {
    let part_opt = conn
        .query_row(
            "SELECT id, name, link FROM parts WHERE id = ?1",
            rusqlite::params![id],
            |row| {
                Ok(Part {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    link: row.get(2)?,
                    exhibit_ids: Vec::new(),
                    notes: Vec::new(),
                })
            },
        )
        .optional()?;

    if let Some(mut part) = part_opt {
        // Fetch associated exhibit IDs
        let mut stmt = conn.prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
        let exhibit_ids_iter = stmt.query_map(rusqlite::params![id], |row| row.get(0))?;
        part.exhibit_ids = exhibit_ids_iter.collect::<rusqlite::Result<Vec<i64>>>()?;

        // Fetch associated notes
        let mut stmt =
            conn.prepare("SELECT id, date, time, message FROM part_notes WHERE part_id = ?1")?;
        let notes_iter = stmt.query_map(rusqlite::params![id], |row| {
            let timestamp = Timestamp {
                date: row.get(1)?,
                time: row.get(2)?,
            };

            Ok(Note {
                id: row.get(0)?,
                timestamp,
                message: row.get(3)?,
            })
        })?;
        part.notes = notes_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

        Ok(Some(part))
    } else {
        Ok(None)
    }
}

/// Handles the GET /parts/<id> endpoint.
///
/// This endpoint retrieves a part with the specified ID from the database.
///
/// # Arguments
/// * `id` - The ID of the part to retrieve.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Part>, ApiError>` - The requested part.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - The part is not found.
/// - A database operation fails.
#[get("/parts/<id>")]
pub async fn get_part_handler(id: i64, db_pool: &State<DbPool>) -> Result<Json<Part>, ApiError> {
    let pool = (*db_pool).clone();

    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        get_part(id, &conn).map_err(|e| {
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
        Some(part) => Ok(Json(part)),
        None => Err(ApiError::NotFound),
    }
}
