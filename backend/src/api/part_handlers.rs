use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Note, Part, UpdatePart};
use crate::repo::part_repo;
use log::{error, info};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rocket::{delete, get, post, put};
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct NewNote {
    pub submitter: String,
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

    part_repo::create_part_note(id, note.submitter, note.message, &pool).await?;

    Ok(())
}

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

#[delete("/parts/<part_id>/notes/<note_id>")]
pub async fn delete_part_note_handler(
    part_id: i64,
    note_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Status, ApiError> {
    let pool = db_pool.inner().clone();

    match part_repo::delete_part_note(part_id, note_id, &pool).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            error!("Failed to delete exhibit note: {}", e);
            Err(ApiError::DatabaseError(
                "Failed to delete exhibit note".to_string(),
            ))
        }
    }
}

/// Handles the DELETE /parts/<id> endpoint.
///
/// This endpoint deletes a part with the specified ID from the database.
///
/// # Arguments
/// * `id` - The ID of the part to delete.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Status, ApiError>` - HTTP status indicating the result of the operation.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The part is not found.
/// - A database operation fails.
#[delete("/parts/<id>")]
pub async fn delete_part_handler(id: i64, db_pool: &State<DbPool>) -> Result<Status, ApiError> {
    let pool = db_pool.inner().clone();

    match part_repo::delete_part(id, &pool).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            error!("Failed to delete part note: {}", e);
            Err(ApiError::DatabaseError(
                "Failed to delete part note".to_string(),
            ))
        }
    }
}

/// Handles the GET /parts/<part_id>/notes/<note_id> endpoint.
///
/// This endpoint retrieves a specific note for a part.
///
/// # Arguments
/// * `part_id` - The ID of the part associated with the note.
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
#[get("/parts/<part_id>/notes/<note_id>")]
pub async fn get_part_note_handler(
    part_id: i64,
    note_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Note>, ApiError> {
    let pool = db_pool.inner().clone();
    let note = part_repo::get_part_note(part_id, note_id, &pool).await?;

    match note {
        Some(note) => Ok(Json(note)),
        None => Err(ApiError::NotFound),
    }
}

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
        None => Ok(Json(Vec::new())),
    }
}

/// Handles the PUT /parts/<id> endpoint.
///
/// This endpoint updates an existing part with the provided data. It updates the part's
/// details as well as its associated exhibits and notes.
///
/// # Arguments
/// * `id` - The ID of the part to update.
/// * `updated_part` - JSON payload containing the updated part data.
/// * `db_pool` - Database connection pool.
///
/// # Returns
/// * `Result<Status, ApiError>` - HTTP status indicating the result of the operation.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The part is not found.
/// - A database operation fails.
#[put("/parts/<id>", format = "json", data = "<updated_part>")]
pub async fn update_part_handler(
    id: i64,
    updated_part: Json<UpdatePart>,
    db_pool: &State<DbPool>,
) -> Result<(), ApiError> {
    let pool = db_pool.inner().clone();

    part_repo::update_part(&id, &updated_part.into_inner(), &pool).await?;

    Ok(())
}
