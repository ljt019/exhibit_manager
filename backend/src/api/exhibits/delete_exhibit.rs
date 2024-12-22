use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::delete;
use rocket::http::Status;
use rocket::State;
use rusqlite::Connection;

/// Deletes an exhibit from the database.
///
/// # Arguments
/// * `id` - The ID of the exhibit to delete
/// * `conn` - Database connection
///
/// # Returns
/// * `rusqlite::Result<usize>` - The number of rows deleted
pub fn delete_exhibit(id: i64, conn: &Connection) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM exhibits WHERE id = ?1", rusqlite::params![id])
}

/// Handles the DELETE /exhibits/<id> endpoint.
///
/// # Arguments
/// * `id` - The ID of the exhibit to delete
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Status, ApiError>` - HTTP status indicating the result
///
/// # Errors
/// Returns `ApiError` if:
/// * Database operations fail
/// * Exhibit is not found
#[delete("/exhibits/<id>")]
pub async fn delete_exhibit_handler(id: i64, db_pool: &State<DbPool>) -> Result<Status, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool
            .get()
            .map_err(|_| ApiError::DatabaseError("Failed to get DB connection".into()))?;
        delete_exhibit(id, &conn).map_err(|e| {
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
        Ok(Status::NoContent)
    } else {
        Err(ApiError::NotFound)
    }
}
