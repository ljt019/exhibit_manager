use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Note, Part, Timestamp};
use log::{error, info};
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Retrieves parts by their IDs from the database.
///
/// This function fetches multiple parts based on the provided list of IDs, including their associated
/// exhibit IDs and notes.
///
/// # Arguments
/// * `ids` - A slice of part IDs to retrieve.
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<Vec<Part>>` - A vector of parts corresponding to the provided IDs.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn get_parts_by_ids(ids: &[i64], conn: &Connection) -> rusqlite::Result<Vec<Part>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    // Create a string of placeholders (?, ?, ?, ...)
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query = format!(
        "SELECT id, name, link FROM parts WHERE id IN ({})",
        placeholders
    );

    let mut stmt = conn.prepare(&query)?;

    // Convert ids to a vector of references for parameter binding
    let id_refs: Vec<&dyn rusqlite::ToSql> =
        ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

    let parts_iter = stmt.query_map(&*id_refs, |row| {
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
            conn.prepare("SELECT id, date, time, message FROM part_notes WHERE part_id = ?1")?;
        let notes_iter = stmt_notes.query_map(rusqlite::params![id], |row| {
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

        parts.push(part);
    }

    Ok(parts)
}

/// Handles the POST /parts/batch endpoint.
///
/// This endpoint retrieves multiple parts based on a list of provided part IDs.
/// It accepts a JSON array of part IDs and returns the corresponding parts.
///
/// # Arguments
/// * `part_ids` - JSON payload containing a list of part IDs.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Part>>, ApiError>` - A JSON array of parts corresponding to the provided IDs.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - The request body is invalid.
/// - A database operation fails.
#[post("/parts/batch", format = "json", data = "<part_ids>")]
pub async fn get_parts_by_ids_handler(
    part_ids: Json<Vec<i64>>,
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Part>>, ApiError> {
    let part_ids = part_ids.into_inner();
    info!("Received /parts/batch request with IDs: {:?}", part_ids);

    if part_ids.is_empty() {
        info!("Empty part_ids received.");
        return Err(ApiError::InvalidRequestBody);
    }

    let pool = (*db_pool).clone();

    let parts_result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        get_parts_by_ids(&part_ids, &conn).map_err(|e| {
            error!("Database error while fetching parts: {}", e);
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
