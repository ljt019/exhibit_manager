use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Part;
use log::error;
use rocket::http::Status;
use rocket::put;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Updates an existing part in the database.
///
/// This function updates the part's details and its associated exhibits and notes.
/// It first updates the main part record, then removes and re-inserts associated exhibits and notes.
///
/// # Arguments
/// * `id` - The ID of the part to update.
/// * `part` - A reference to the `Part` struct containing updated data.
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<usize>` - The number of rows affected.
///
/// # Errors
/// Returns a `rusqlite::Error` if any database operation fails.
pub fn update_part(id: i64, part: &Part, conn: &Connection) -> rusqlite::Result<usize> {
    let affected = conn.execute(
        "UPDATE parts SET name = ?1, link = ?2 WHERE id = ?3",
        rusqlite::params![part.name, part.link, id],
    )?;

    // Update associated exhibits: Remove existing and add new associations
    conn.execute(
        "DELETE FROM exhibit_parts WHERE part_id = ?1",
        rusqlite::params![id],
    )?;
    for exhibit_id in &part.exhibit_ids {
        conn.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            rusqlite::params![exhibit_id, id],
        )?;
    }

    // Update notes: Remove existing and add new notes
    conn.execute(
        "DELETE FROM part_notes WHERE part_id = ?1",
        rusqlite::params![id],
    )?;
    for note in &part.notes {
        conn.execute(
            "INSERT INTO part_notes (part_id, date, time, message) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                id,
                &note.timestamp.date,
                &note.timestamp.time,
                &note.message
            ],
        )?;
    }

    Ok(affected)
}

/// Handles the PUT /parts/<id> endpoint.
///
/// This endpoint updates an existing part with the provided data. It updates the part's
/// details as well as its associated exhibits and notes.
///
/// # Arguments
/// * `id` - The ID of the part to update.
/// * `updated_part` - JSON payload containing the updated part data.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Status, ApiError>` - HTTP status indicating the result of the operation.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - The part is not found.
/// - A database operation fails.
#[put("/parts/<id>", format = "json", data = "<updated_part>")]
pub async fn update_part_handler(
    id: i64,
    updated_part: Json<Part>,
    db_pool: &State<DbPool>,
) -> Result<Status, ApiError> {
    let pool = (*db_pool).clone();

    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        update_part(id, &updated_part.into_inner(), &conn).map_err(|e| {
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
