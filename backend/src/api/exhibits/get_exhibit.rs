use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::models::Note;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, OptionalExtension, Result as SqliteResult};

pub fn get_exhibit(id: i64, db_conn: &DbConnection) -> SqliteResult<Option<Exhibit>> {
    let exhibit_opt = db_conn.0
            .query_row(
                "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date 
                 FROM exhibits WHERE id = ?1",
                params![id],
                |row| {
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
                        part_ids: Vec::new(), // To be populated
                        notes: Vec::new(),    // To be populated
                    })
                },
            )
            .optional()?;

    if let Some(mut exhibit) = exhibit_opt {
        // Fetch associated part IDs
        let mut stmt = db_conn
            .0
            .prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
        let part_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
        exhibit.part_ids = part_ids_iter.collect::<Result<Vec<i64>, _>>()?;

        // Fetch associated notes
        let mut stmt = db_conn
            .0
            .prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt.query_map(params![id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;
        exhibit.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

        Ok(Some(exhibit))
    } else {
        Ok(None)
    }
}

/// Handles the GET /exhibits/<id> endpoint
#[get("/exhibits/<id>")]
pub async fn get_exhibit_handler(
    id: i64,
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<Exhibit>, ApiError> {
    let db_conn = db.lock().await;

    match get_exhibit(id, &*db_conn) {
        Ok(Some(exhibit)) => Ok(Json(exhibit)),
        Ok(None) => Err(ApiError::NotFound),
        Err(e) => Err(ApiError::DatabaseError(e.to_string())),
    }
}
