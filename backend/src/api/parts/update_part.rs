use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Part;
use crate::repo::part_repo;
use rocket::put;
use rocket::serde::json::Json;
use rocket::State;

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
/// - The part is not found.
/// - A database operation fails.
#[put("/parts/<id>", format = "json", data = "<updated_part>")]
pub async fn update_part_handler(
    id: i64,
    updated_part: Json<Part>,
    db_pool: &State<DbPool>,
) -> Result<(), ApiError> {
    let pool = db_pool.inner().clone();

    part_repo::update_part(&id, &updated_part.into_inner(), &pool).await?;

    Ok(())
}
