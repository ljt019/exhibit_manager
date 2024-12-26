use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::delete;
use rocket::http::Status;
use rocket::State;
use rusqlite::Connection;

pub fn delete_part_note(part_id: i64, note_id: i64, conn: &Connection) -> rusqlite::Result<usize> {
    conn.execute(
        "DELETE FROM part_notes WHERE part_id = ?1 AND note_id = ?2",
        rusqlite::params![part_id, note_id],
    )
}

#[delete("/parts/<part_id>/notes/<note_id>")]
pub async fn delete_part_note_handler(
    part_id: i64,
    note_id: i64,
    db_pool: &State<DbPool>,
) -> Result<Status, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool
            .get()
            .map_err(|_| ApiError::DatabaseError("Failed to get DB connection".into()))?;
        delete_part_note(part_id, note_id, &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    if result > 0 {
        Ok(Status::NoContent)
    } else {
        Err(ApiError::NotFound)
    }
}
