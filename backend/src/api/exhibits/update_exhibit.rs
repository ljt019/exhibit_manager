use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::repo::exhibit_repo;
use log::error;
use rocket::put;
use rocket::serde::json::Json;
use rocket::State;

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
/// - The exhibit is not found.
/// - A database operation fails.
#[put("/exhibits/<id>", format = "json", data = "<updated_exhibit>")]
pub async fn update_exhibit_handler(
    id: i64,
    updated_exhibit: Json<Exhibit>,
    db_pool: &State<DbPool>,
) -> Result<(), ApiError> {
    let pool = db_pool.inner().clone();
    let exhibit = updated_exhibit.into_inner();

    // Update the exhibit
    exhibit_repo::update_exhibit(&id, &exhibit, &pool)
        .await
        .map_err(|e| {
            error!("Failed to update exhibit: {}", e);
            ApiError::DatabaseError("Failed to update exhibit".into())
        })?;

    Ok(())
}
