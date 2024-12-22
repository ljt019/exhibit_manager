use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::delete;
use rocket::http::Status;
use rocket::State;
use rusqlite::Connection;

/// Deletes a part from the database.
///
/// This function removes a part with the specified ID from the `parts` table.
///
/// # Arguments
/// * `id` - The ID of the part to delete.
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<usize>` - The number of rows deleted.
///
/// # Errors
/// Returns a `rusqlite::Error` if the delete operation fails.
pub fn delete_part(id: i64, conn: &Connection) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM parts WHERE id = ?1", rusqlite::params![id])
}

/// Handles the DELETE /parts/<id> endpoint.
///
/// This endpoint deletes a part with the specified ID from the database.
///
/// # Arguments
/// * `id` - The ID of the part to delete.
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
#[delete("/parts/<id>")]
pub async fn delete_part_handler(id: i64, db_pool: &State<DbPool>) -> Result<Status, ApiError> {
    let pool = (*db_pool).clone();

    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        delete_part(id, &conn).map_err(|e| {
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
