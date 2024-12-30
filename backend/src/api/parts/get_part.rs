use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Part;
use crate::repo::part_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /parts/<id> endpoint.
///
/// This endpoint retrieves a part with the specified ID from the database.
///
/// # Arguments
/// * `id` - The ID of the part to retrieve.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Part>, ApiError>` - The requested part.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The part is not found.
/// - A database operation fails.
#[get("/parts/<id>")]
pub async fn get_part_handler(id: i64, db_pool: &State<DbPool>) -> Result<Json<Part>, ApiError> {
    let pool = db_pool.inner().clone();
    let part = part_repo::get_part(id, &pool).await?;

    match part {
        Some(part) => Ok(Json(part)),
        None => Err(ApiError::NotFound),
    }
}
