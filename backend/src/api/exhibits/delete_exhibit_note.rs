use crate::db::DbPool;
use crate::errors::ApiError;
use crate::repo::exhibit_repo;
use log::error;
use rocket::delete;
use rocket::http::Status;
use rocket::State;

#[delete("/exhibits/<exhibit_id>/notes/<note_id>")]
pub async fn delete_exhibit_note_handler(
    exhibit_id: i64,
    note_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Status, ApiError> {
    let pool = db_pool.inner().clone();

    match exhibit_repo::delete_exhibit_note(exhibit_id, note_id, &pool).await {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            error!("Failed to delete exhibit note: {}", e);
            Err(ApiError::DatabaseError(
                "Failed to delete exhibit note".to_string(),
            ))
        }
    }
}
