use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{FullName, Jotform};
use log::error;
use rocket::delete;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

pub fn delete_jotform(id: i64, conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM jotforms WHERE id = ?1", [id])?;
    Ok(())
}

#[delete("/jotforms/<id>")]
pub async fn delete_jotform_handler(id: i64, db_pool: &State<DbPool>) -> Result<(), ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let _result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        delete_jotform(id, &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(())
}
