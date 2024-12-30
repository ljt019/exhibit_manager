use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::exhibit_repo;
use log::error;
use rocket::delete;
use rocket::http::Status;
use rocket::State;

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
/// * The exhibit is not found
/// * A database operation fails
#[delete("/exhibits/<id>")]
pub async fn delete_exhibit_handler(id: i64, db_pool: &State<DbPool>) -> Result<Status, ApiError> {
    let pool = db_pool.inner().clone();

    match exhibit_repo::get_exhibit(id, &pool).await {
        Ok(Some(_)) => match exhibit_repo::delete_exhibit(id, &pool).await {
            Ok(_) => Ok(Status::NoContent),
            Err(e) => {
                error!("Failed to delete exhibit: {}", e);
                Err(ApiError::DatabaseError(
                    "Failed to delete exhibit".to_string(),
                ))
            }
        },
        Ok(None) => Err(ApiError::NotFound),
        Err(e) => {
            error!("Failed to get exhibit: {}", e);
            Err(ApiError::DatabaseError("Failed to get exhibit".to_string()))
        }
    }
}
