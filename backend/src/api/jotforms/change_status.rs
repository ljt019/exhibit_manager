use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rusqlite::params;
use rusqlite::Connection;

#[derive(Debug, Deserialize)]
pub struct ChangeStatusRequest {
    pub new_status: String,
}

pub fn change_status(id: i64, new_status: &str, conn: &Connection) -> Result<(), rusqlite::Error> {
    // Validate the new status value
    match new_status {
        "Open" | "InProgress" | "Closed" => {}
        _ => {
            return Err(rusqlite::Error::InvalidQuery);
        }
    }

    conn.execute(
        "UPDATE jotforms SET status = ? WHERE id = ?",
        params![new_status, id],
    )?;
    Ok(())
}

#[post("/jotforms/<id>/status", data = "<data>")]
pub async fn change_status_handler(
    db_pool: &State<DbPool>,
    id: i64,
    data: Json<ChangeStatusRequest>,
) -> Result<Json<String>, ApiError> {
    let new_status = data.new_status.trim().to_string();
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let _ = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        change_status(id, &new_status, &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json("Status successfully updated".to_string()))
}
