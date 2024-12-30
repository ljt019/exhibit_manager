use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::repo::exhibit_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /exhibits endpoint.
///
/// This endpoint retrieves a list of all exhibits from the database.
///
/// # Arguments
/// * `db_pool` - A reference to the database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Exhibit>>, ApiError>` - A JSON array of exhibits if successful.
///
/// # Errors
/// Returns an `ApiError` if a database operation fails.
#[get("/exhibits")]
pub async fn list_exhibits_handler(
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Exhibit>>, ApiError> {
    let pool = db_pool.inner().clone();
    let exhibits = exhibit_repo::get_all_exhibits(&pool).await?;

    match exhibits {
        Some(exhibits) => Ok(Json(exhibits)),
        None => Err(ApiError::NotFound),
    }
}
