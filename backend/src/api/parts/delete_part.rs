use crate::db::DbConnection;
use crate::errors::ApiError;
use rocket::http::Status;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

pub fn delete_part(id: i64, db_conn: &DbConnection) -> SqliteResult<usize> {
    db_conn
        .access()
        .execute("DELETE FROM parts WHERE id = ?1", params![id])
}

/// Handles the DELETE /parts/<id> endpoint
#[delete("/parts/<id>")]
pub async fn delete_part_handler(
    id: i64,
    db: &State<Mutex<DbConnection>>,
) -> Result<Status, ApiError> {
    let db_conn = db.lock().await;

    match delete_part(id, &*db_conn) {
        Ok(deleted) if deleted > 0 => Ok(Status::NoContent),
        Ok(_) => Err(ApiError::NotFound),
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
