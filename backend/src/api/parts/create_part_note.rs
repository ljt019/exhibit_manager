use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::part_repo;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct NewNote {
    #[validate(length(min = 1, message = "Note cannot be empty"))]
    pub message: String,
}

/// Creates a new note with associated part.
///
/// # Arguments
/// * `new_note` - JSON payload containing the note data
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Json<i64>, ApiError>` - The ID of the newly created note
///
/// # Errors
/// Returns `ApiError` if:
/// * Database operations fail
/// * Input validation fails
#[post("/parts/<id>/notes", format = "json", data = "<new_note>")]
pub async fn create_part_note_handler(
    id: i64,
    new_note: Json<NewNote>,
    db_pool: &State<DbPool>,
) -> Result<(), ApiError> {
    let note = new_note.into_inner();
    let pool = db_pool.inner().clone();

    part_repo::create_part_note(id, note.message, &pool).await?;

    Ok(())
}
