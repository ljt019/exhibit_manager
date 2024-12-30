use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::repo::exhibit_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /exhibits/<id> endpoint.
///
/// # Arguments
/// * `id` - The ID of the exhibit to retrieve
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Json<Exhibit>, ApiError>` - The requested exhibit
///
/// # Errors
/// Returns an `ApiError` if:
/// - The exhibit is not found.
/// - A database operation fails.
#[get("/exhibits/<id>")]
pub async fn get_exhibit_handler(
    id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Exhibit>, ApiError> {
    let pool = db_pool.inner();
    let exhibit = exhibit_repo::get_exhibit(id, &pool).await?;

    match exhibit {
        Some(exhibit) => Ok(Json(exhibit)),
        None => Err(ApiError::NotFound),
    }
}
