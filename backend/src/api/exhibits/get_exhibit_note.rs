use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Note;
use crate::repo::exhibit_repo;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

/// Handles the GET /exhibits/<exhibit_id>/notes/<note_id> endpoint.
///
/// This endpoint retrieves a specific note for an exhibit.
///
/// # Arguments
/// * `exhibit_id` - The ID of the exhibit associated with the note.
/// * `note_id` - The ID of the note to retrieve.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Json<Note>, ApiError>` - The retrieved note or an error if not found.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The note is not found.
/// - A database operation fails.
#[get("/exhibits/<exhibit_id>/notes/<note_id>")]
pub async fn get_exhibit_note_handler(
    exhibit_id: i64,
    note_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Note>, ApiError> {
    let pool = db_pool.inner().clone();
    let note = exhibit_repo::get_exhibit_note(exhibit_id, note_id, &pool).await?;

    match note {
        Some(note) => Ok(Json(note)),
        None => Err(ApiError::NotFound),
    }
}
