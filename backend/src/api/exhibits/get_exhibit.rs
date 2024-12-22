use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Exhibit, Note};
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

/// Retrieves an exhibit from the database.
///
/// # Arguments
/// * `id` - The ID of the exhibit to retrieve
/// * `conn` - Database connection
///
/// # Returns
/// * `rusqlite::Result<Option<Exhibit>>` - The exhibit if found
pub fn get_exhibit(id: i64, conn: &Connection) -> rusqlite::Result<Option<Exhibit>> {
    let exhibit_opt = conn
        .query_row(
            "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date 
             FROM exhibits WHERE id = ?1",
            rusqlite::params![id],
            |row| {
                Ok(Exhibit {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    cluster: row.get(2)?,
                    location: row.get(3)?,
                    status: row.get(4)?,
                    image_url: row.get(5)?,
                    sponsor_name: row.get(6)?,
                    sponsor_start_date: row.get(7)?,
                    sponsor_end_date: row.get(8)?,
                    part_ids: Vec::new(), // To be populated
                    notes: Vec::new(),    // To be populated
                })
            },
        )
        .optional()?;

    if let Some(mut exhibit) = exhibit_opt {
        // Fetch associated part IDs
        let mut stmt = conn.prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
        let part_ids_iter = stmt.query_map(rusqlite::params![id], |row| row.get(0))?;
        exhibit.part_ids = part_ids_iter.collect::<rusqlite::Result<Vec<i64>>>()?;

        // Fetch associated notes
        let mut stmt =
            conn.prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt.query_map(rusqlite::params![id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;
        exhibit.notes = notes_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

        Ok(Some(exhibit))
    } else {
        Ok(None)
    }
}

/// Handles the GET /exhibits/<id> endpoint.
///
/// # Arguments
/// * `id` - The ID of the exhibit to retrieve
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Json<Exhibit>, ApiError>` - The requested exhibit
///
/// # Errors
/// Returns `ApiError` if:
/// * Database operations fail
/// * Exhibit is not found
#[get("/exhibits/<id>")]
pub async fn get_exhibit_handler(
    id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Exhibit>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        get_exhibit(id, &conn).map_err(|e| {
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
        Some(exhibit) => Ok(Json(exhibit)),
        None => Err(ApiError::NotFound),
    }
}
