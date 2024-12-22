use crate::db::DbConnection;
use crate::errors::ApiError;
use rocket::http::Status;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

pub fn delete_exhibit(id: i64, db_conn: &DbConnection) -> SqliteResult<usize> {
    db_conn
        .0
        .execute("DELETE FROM exhibits WHERE id = ?1", params![id])
}

/// Handles the DELETE /exhibits/<id> endpoint
#[delete("/exhibits/<id>")]
pub async fn delete_exhibit_handler(
    id: i64,
    db: &State<Mutex<DbConnection>>,
) -> Result<Status, ApiError> {
    let db_conn = db.lock().await;

    match delete_exhibit(id, &*db_conn) {
        Ok(deleted) if deleted > 0 => Ok(Status::NoContent),
        Ok(_) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
