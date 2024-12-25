use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use log::error;
use rocket::http::Status;
use rocket::put;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Updates an existing exhibit in the database.
///
/// This function updates the exhibit's details and its associated parts and notes.
/// It first updates the main exhibit record, then removes and re-inserts associated parts and notes.
///
/// # Arguments
/// * `id` - The ID of the exhibit to update.
/// * `exhibit` - A reference to the `Exhibit` struct containing updated data.
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<usize>` - The number of rows affected.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn update_exhibit(id: i64, exhibit: &Exhibit, conn: &Connection) -> rusqlite::Result<usize> {
    let affected = conn.execute(
        "UPDATE exhibits 
             SET name = ?1, cluster = ?2, location = ?3, status = ?4, image_url = ?5, 
                 sponsor_name = ?6, sponsor_start_date = ?7, sponsor_end_date = ?8 
             WHERE id = ?9",
        rusqlite::params![
            exhibit.name,
            exhibit.cluster,
            exhibit.location,
            exhibit.status,
            exhibit.image_url,
            exhibit.sponsor_name,
            exhibit.sponsor_start_date,
            exhibit.sponsor_end_date,
            id
        ],
    )?;

    // Update associated parts: Remove existing and add new associations
    conn.execute(
        "DELETE FROM exhibit_parts WHERE exhibit_id = ?1",
        rusqlite::params![id],
    )?;
    for part_id in &exhibit.part_ids {
        conn.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            rusqlite::params![id, part_id],
        )?;
    }

    // Update notes: Remove existing and add new notes
    conn.execute(
        "DELETE FROM exhibit_notes WHERE exhibit_id = ?1",
        rusqlite::params![id],
    )?;
    for note in &exhibit.notes {
        conn.execute(
            "INSERT INTO exhibit_notes (exhibit_id, timestamp, message) VALUES (?1, ?2, ?3)",
            rusqlite::params![id, &note.timestamp, &note.message],
        )?;
    }

    Ok(affected)
}

/// Handles the PUT /exhibits/<id> endpoint.
///
/// This endpoint updates an existing exhibit with the provided data. It updates the exhibit's
/// details as well as its associated parts and notes.
///
/// # Arguments
/// * `id` - The ID of the exhibit to update.
/// * `updated_exhibit` - JSON payload containing the updated exhibit data.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Status, ApiError>` - HTTP status indicating the result of the operation.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - The exhibit is not found.
/// - A database operation fails.
#[put("/exhibits/<id>", format = "json", data = "<updated_exhibit>")]
pub async fn update_exhibit_handler(
    id: i64,
    updated_exhibit: Json<Exhibit>,
    db_pool: &State<DbPool>,
) -> Result<Status, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        update_exhibit(id, &updated_exhibit.into_inner(), &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    if result > 0 {
        Ok(Status::Ok)
    } else {
        Err(ApiError::NotFound)
    }
}
