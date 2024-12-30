use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Note;
use crate::repo::exhibit_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /exhibits/<exhibit_id>/notes endpoint.
///
/// This endpoint retrieves all notes associated with a specific exhibit.
///
/// # Arguments
/// * `exhibit_id` - The ID of the exhibit to retrieve notes for.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Vec<Note>>, ApiError>` - A JSON array of notes associated with the exhibit.
///
/// # Errors
/// Returns an `ApiError` if a database operation fails.
#[get("/exhibits/<exhibit_id>/notes")]
pub async fn list_exhibit_notes_handler(
    exhibit_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Vec<Note>>, ApiError> {
    let pool = db_pool.inner().clone();
    let notes = exhibit_repo::get_all_exhibit_notes(exhibit_id, &pool).await?;

    match notes {
        Some(notes) => Ok(Json(notes)),
        None => Err(ApiError::NotFound),
    }
}
