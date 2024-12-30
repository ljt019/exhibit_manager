use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Note;
use crate::repo::part_repo;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;

#[derive(Deserialize)]
pub struct NewPart {
    pub name: String,
    pub link: String,
    pub exhibit_ids: Vec<i64>,
    pub notes: Vec<Note>,
}

/// Handles the POST /parts endpoint.
///
/// This endpoint creates a new part with associated exhibits and notes.
/// It processes the incoming JSON payload and returns the ID of the newly created part.
///
/// # Arguments
/// * `new_part` - JSON payload containing the part data.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<i64>, ApiError>` - The ID of the newly created part.
///
/// # Errors
/// Returns an `ApiError` if:
/// - A database operation fails.
#[post("/parts", format = "json", data = "<new_part>")]
pub async fn create_part_handler(
    new_part: Json<NewPart>,
    db_pool: &State<DbPool>,
) -> Result<(), ApiError> {
    let pool = db_pool.inner().clone();

    part_repo::create_part(&new_part.into_inner(), &pool).await?;

    Ok(())
}
