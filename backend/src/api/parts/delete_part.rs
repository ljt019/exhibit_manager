use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::part_repo;
use log::error;
use rocket::delete;
use rocket::http::Status;
use rocket::State;

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
/// - The part is not found.
/// - A database operation fails.
#[delete("/parts/<id>")]
pub async fn delete_part_handler(id: i64, db_pool: &State<DbPool>) -> Result<Status, ApiError> {
    let pool = db_pool.inner().clone();

    match part_repo::delete_part(id, &pool).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            error!("Failed to delete part note: {}", e);
            Err(ApiError::DatabaseError(
                "Failed to delete part note".to_string(),
            ))
        }
    }
}
