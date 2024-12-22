use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Note;
use crate::models::Part;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::OptionalExtension;
use rusqlite::{params, Result as SqliteResult};

fn get_part(id: i64, db_conn: &DbConnection) -> SqliteResult<Option<Part>> {
    let part_opt = db_conn
        .0
        .query_row(
            "SELECT id, name, link FROM parts WHERE id = ?1",
            params![id],
            |row| {
                Ok(Part {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    link: row.get(2)?,
                    exhibit_ids: Vec::new(),
                    notes: Vec::new(),
                })
            },
        )
        .optional()?;

    if let Some(mut part) = part_opt {
        // Fetch associated exhibit IDs
        let mut stmt = db_conn
            .0
            .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
        let exhibit_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
        part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

        // Fetch associated notes
        let mut stmt = db_conn
            .0
            .prepare("SELECT timestamp, note FROM part_notes WHERE part_id = ?1")?;
        let notes_iter = stmt.query_map(params![id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;
        part.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

        Ok(Some(part))
    } else {
        Ok(None)
    }
}

/// Handles the GET /parts/<id> endpoint
#[get("/parts/<id>")]
pub async fn get_part_handler(
    id: i64,
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<Part>, ApiError> {
    let db_conn = db.lock().await;

    match get_part(id, &*db_conn) {
        Ok(Some(part)) => Ok(Json(part)),
        Ok(None) => Err(ApiError::NotFound),
        Err(e) => Err(ApiError::DatabaseError(e.to_string())),
    }
}
