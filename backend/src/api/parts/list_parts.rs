use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Part;
use crate::repo::part_repo;
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /parts endpoint.
///
/// This endpoint retrieves a list of all parts from the database.
///
/// # Arguments
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Part>>, ApiError>` - A JSON array of parts.
///
/// # Errors
/// Returns an `ApiError` if a database operation fails.
#[get("/parts")]
pub async fn list_parts_handler(db_pool: &State<DbPool>) -> Result<Json<Vec<Part>>, ApiError> {
    let pool = db_pool.inner().clone();
    let parts = part_repo::get_all_parts(&pool).await?;

    match parts {
        Some(parts) => Ok(Json(parts)),
        None => {
            error!("No parts found in the database.");
            Err(ApiError::NotFound)
        }
    }
}
