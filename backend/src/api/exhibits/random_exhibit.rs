use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::{Exhibit, Note};
use rand::seq::SliceRandom;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

/// Fetches a single random exhibit from the database.
pub fn random_exhibit(db_conn: &DbConnection) -> SqliteResult<Exhibit> {
    // Prepare the SQL statement to select all exhibits
    let mut stmt = db_conn.0.prepare(
        "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date FROM exhibits"
    )?;

    // Map each row to an Exhibit struct
    let exhibits_iter = stmt.query_map([], |row| {
        Ok(Exhibit {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            cluster: row.get(2)?,
            location: row.get(3)?,
            status: row.get(4)?,
            image_url: row.get(5)?,
            sponsor_name: row.get(6)?,
            sponsor_start_date: row.get(7)?,
            sponsor_end_date: row.get(8)?,
            part_ids: Vec::new(),
            notes: Vec::new(),
        })
    })?;

    let mut exhibits = Vec::new();
    for exhibit_res in exhibits_iter {
        let mut exhibit = exhibit_res?;
        let id = exhibit.id.unwrap();

        // Fetch associated part IDs
        let mut stmt_parts = db_conn
            .0
            .prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
        let part_ids_iter = stmt_parts.query_map(params![id], |row| row.get(0))?;
        exhibit.part_ids = part_ids_iter.collect::<Result<Vec<i64>, _>>()?;

        // Fetch associated notes
        let mut stmt_notes = db_conn
            .0
            .prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt_notes.query_map(params![id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;
        exhibit.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

        exhibits.push(exhibit);
    }

    // Check if any exhibits were found
    if exhibits.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    // Select a random exhibit
    let random_exhibit = exhibits.choose(&mut rand::thread_rng()).unwrap().clone();
    Ok(random_exhibit)
}

/// Handles the GET /exhibits/random endpoint
#[get("/exhibits/random")]
pub async fn handle_random_exhibit(
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<Exhibit>, ApiError> {
    // Acquire a lock on the database connection
    let db_conn = db.lock().await;

    // Call the `random_exhibit` function to get a random exhibit
    match random_exhibit(&*db_conn) {
        Ok(exhibit) => Ok(Json(exhibit)),
        // If no exhibits are found, return a 404 Not Found error
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(ApiError::NotFound),
        // For other database errors, return a generic database error
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
