use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Note;
use crate::repo::part_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /parts/<part_id>/notes endpoint.
///
/// This endpoint retrieves all notes associated with a specific part.
///
/// # Arguments
/// * `part_id` - The ID of the part to retrieve notes for.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Note>>, ApiError>` - A JSON array of notes associated with the part.
///
/// # Errors
/// Returns an `ApiError` if a database operation fails.
#[get("/parts/<part_id>/notes")]
pub async fn list_part_notes_handler(
    part_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Note>>, ApiError> {
    let pool = db_pool.inner().clone();
    let notes = part_repo::get_all_part_notes(part_id, &pool).await?;

    match notes {
        Some(notes) => Ok(Json(notes)),
        None => Err(ApiError::NotFound),
    }
}
