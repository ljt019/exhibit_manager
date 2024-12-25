use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Exhibit, Note};
use log::error;
use rand::seq::SliceRandom;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Fetches a single random exhibit from the database.
///
/// This function retrieves all exhibits, collects them into a vector, and selects one at random.
///
/// # Arguments
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<Exhibit>` - A randomly selected exhibit.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails or if no exhibits are found.
pub fn random_exhibit(conn: &Connection) -> rusqlite::Result<Exhibit> {
    // Prepare the SQL statement to select all exhibits
    let mut stmt = conn.prepare(
        "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date FROM exhibits"
    )?;

    // Map each row to an Exhibit struct
    let exhibits_iter = stmt.query_map([], |row| {
        Ok(Exhibit {
            id: row.get(0)?,
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
        let id = exhibit.id;

        // Fetch associated part IDs
        let mut stmt_parts =
            conn.prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
        let part_ids_iter = stmt_parts.query_map(rusqlite::params![id], |row| row.get(0))?;
        exhibit.part_ids = part_ids_iter.collect::<rusqlite::Result<Vec<i64>>>()?;

        // Fetch associated notes
        let mut stmt_notes =
            conn.prepare("SELECT timestamp, message FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt_notes.query_map(rusqlite::params![id], |row| {
            Ok(Note {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                message: row.get(2)?,
            })
        })?;
        exhibit.notes = notes_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

        exhibits.push(exhibit);
    }

    // Check if any exhibits were found
    if exhibits.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    // Select a random exhibit
    let random_exhibit = exhibits.choose(&mut rand::thread_rng()).unwrap().clone();
    Ok(random_exhibit)
}

/// Handles the GET /exhibits/random endpoint.
///
/// This endpoint retrieves a random exhibit from the database.
///
/// # Arguments
/// * `db_pool` - A reference to the database connection pool.
///
/// # Returns
/// * `Result<Json<Exhibit>, ApiError>` - The randomly selected exhibit.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - No exhibits are found.
/// - A database operation fails.
#[get("/exhibits/random")]
pub async fn handle_random_exhibit(db_pool: &State<DbPool>) -> Result<Json<Exhibit>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        random_exhibit(&conn).map_err(|e| {
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
