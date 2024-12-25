use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use log::error;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Inserts a new exhibit into the database and returns its ID.
pub fn create_exhibit(exhibit: &Exhibit, conn: &Connection) -> rusqlite::Result<i64> {
    // Insert the exhibit
    conn.execute(
        "INSERT INTO exhibits (name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            exhibit.name,
            exhibit.cluster,
            exhibit.location,
            exhibit.status,
            exhibit.image_url,
            exhibit.sponsor_name,
            exhibit.sponsor_start_date,
            exhibit.sponsor_end_date,
        ],
    )?;
    let exhibit_id = conn.last_insert_rowid();

    // Associate parts with the exhibit
    for part_id in &exhibit.part_ids {
        conn.execute(
            "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
            rusqlite::params![exhibit_id, part_id],
        )?;
    }

    // Insert notes related to the exhibit
    for note in &exhibit.notes {
        conn.execute(
            "INSERT INTO exhibit_notes (exhibit_id, timestamp, message) VALUES (?1, ?2, ?3)",
            rusqlite::params![exhibit_id, &note.timestamp, &note.message],
        )?;
    }

    Ok(exhibit_id)
}

/// Creates a new exhibit with associated parts and notes.
///
/// # Arguments
/// * `new_exhibit` - JSON payload containing the exhibit data
/// * `db_pool` - Database connection pool
///
/// # Returns
/// * `Result<Json<i64>, ApiError>` - The ID of the newly created exhibit
///
/// # Errors
/// Returns `ApiError` if:
/// * Database operations fail
/// * Input validation fails
#[post("/exhibits", format = "json", data = "<new_exhibit>")]
pub async fn create_exhibit_handler(
    new_exhibit: Json<Exhibit>,
    db_pool: &State<DbPool>,
) -> Result<Json<i64>, ApiError> {
    let exhibit = new_exhibit.into_inner();
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().expect("Failed to get DB connection from pool");
        create_exhibit(&exhibit, &conn)
    })
    .await
    .map_err(|e| {
        error!("Task panicked: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(result))
}
