use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Part;
use crate::repo::part_repo;
use log::info;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the POST /parts/batch endpoint.
///
/// This endpoint retrieves multiple parts based on a list of provided part IDs.
/// It accepts a JSON array of part IDs and returns the corresponding parts.
///
/// # Arguments
/// * `part_ids` - JSON payload containing a list of part IDs.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Part>>, ApiError>` - A JSON array of parts corresponding to the provided IDs.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The request body is invalid.
/// - A database operation fails.
#[post("/parts/batch", format = "json", data = "<part_ids>")]
pub async fn get_parts_by_ids_handler(
    part_ids: Json<Vec<i64>>,
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Part>>, ApiError> {
    let part_ids = part_ids.into_inner();
    info!("Received /parts/batch request with IDs: {:?}", part_ids);

    if part_ids.is_empty() {
        info!("Empty part_ids received.");
        return Err(ApiError::InvalidRequestBody);
    }

    let pool = db_pool.inner().clone();
    let parts = part_repo::get_parts_by_ids(part_ids, &pool).await?;

    match parts {
        Some(parts) => Ok(Json(parts)),
        None => Err(ApiError::NotFound),
    }
}
