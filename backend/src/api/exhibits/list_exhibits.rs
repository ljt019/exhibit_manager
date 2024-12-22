use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Exhibit, Note};
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Retrieves all exhibits from the database.
///
/// This function fetches all exhibits along with their associated part IDs and notes.
///
/// # Arguments
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<Vec<Exhibit>>` - A vector of exhibits if the operation is successful.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn list_exhibits(conn: &Connection) -> rusqlite::Result<Vec<Exhibit>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date FROM exhibits"
    )?;
    let exhibits_iter = stmt.query_map([], |row| {
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
            part_ids: Vec::new(),
            notes: Vec::new(),
        })
    })?;

    let mut exhibits = Vec::new();
    for exhibit_res in exhibits_iter {
        let mut exhibit = exhibit_res?;
        let id = exhibit.id.unwrap();

        // Fetch associated part IDs
        let mut stmt_parts =
            conn.prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
        let part_ids_iter = stmt_parts.query_map(rusqlite::params![id], |row| row.get(0))?;
        exhibit.part_ids = part_ids_iter.collect::<rusqlite::Result<Vec<i64>>>()?;

        // Fetch associated notes
        let mut stmt_notes =
            conn.prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt_notes.query_map(rusqlite::params![id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;
        exhibit.notes = notes_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

        exhibits.push(exhibit);
    }

    Ok(exhibits)
}

/// Handles the GET /exhibits endpoint.
///
/// This endpoint retrieves a list of all exhibits from the database.
///
/// # Arguments
/// * `db_pool` - A reference to the database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Exhibit>>, ApiError>` - A JSON array of exhibits if successful.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - A database operation fails.
#[get("/exhibits")]
pub async fn list_exhibits_handler(
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Exhibit>>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        list_exhibits(&conn).map_err(|e| {
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
