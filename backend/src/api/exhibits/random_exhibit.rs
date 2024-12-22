use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::models::Note;
use rand::seq::SliceRandom;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

pub fn list_exhibits(db_conn: &DbConnection) -> SqliteResult<Vec<Exhibit>> {
    let mut stmt = db_conn.0.prepare(
            "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date FROM exhibits"
        )?;
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

    Ok(exhibits)
}

/// Handles the GET /exhibits/random endpoint
#[get("/exhibits/random")]
pub async fn handle_random_exhibit(
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<Exhibit>, ApiError> {
    let db_conn = db.lock().await;

    match list_exhibits(&*db_conn) {
        Ok(exhibits) => {
            if exhibits.is_empty() {
                return Err(ApiError::NotFound);
            }
            let random_exhibit = exhibits.choose(&mut rand::thread_rng()).unwrap().clone();
            Ok(Json(random_exhibit))
        }
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
