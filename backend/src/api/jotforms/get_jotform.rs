use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{FullName, Jotform};
use log::error;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Retrieves all jotforms from the database.
pub fn get_jotform(id: i64, conn: &Connection) -> rusqlite::Result<Jotform> {
    let mut stmt = conn.prepare(
        "SELECT id, submitter_first_name, submitter_last_name, submission_date, submission_time, location, exhibit_name, description, priority_level, department, status FROM jotforms WHERE id = ?1"
    )?;

    let mut jotform_iter = stmt.query_map(rusqlite::params![id], |row| {
        let submission_date_raw = row.get(3)?;
        let submission_time_raw = row.get(4)?;

        let submitter_first_name = row.get(1)?;
        let submitter_last_name = row.get(2)?;

        let submission_date = crate::models::SubmissionDate {
            date: submission_date_raw,
            time: submission_time_raw,
        };

        let submitter_name = FullName {
            first: submitter_first_name,
            last: submitter_last_name,
        };

        Ok(Jotform {
            id: row.get(0)?,
            submitter_name: submitter_name,
            created_at: submission_date,
            location: row.get(5)?,
            exhibit_name: row.get(6)?,
            description: row.get(7)?,
            priority_level: row.get(8)?,
            department: row.get(9)?,
            status: row.get(10)?,
        })
    })?;

    let jotform = jotform_iter
        .next()
        .ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)??;

    Ok(jotform)
}

#[get("/jotforms/<id>")]
pub async fn get_jotform_handler(
    id: i64,
    db_pool: &State<DbPool>,
) -> Result<Json<Jotform>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        get_jotform(id, &conn).map_err(|e| {
            error!("Database error: {}", e);
            ApiError::DatabaseError("Database Error".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(result))
}