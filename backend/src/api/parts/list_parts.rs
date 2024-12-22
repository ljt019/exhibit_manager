use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Note;
use crate::models::Part;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

pub fn list_parts(db_conn: &DbConnection) -> SqliteResult<Vec<Part>> {
    let mut stmt = db_conn.0.prepare("SELECT id, name, link FROM parts")?;
    let parts_iter = stmt.query_map([], |row| {
        Ok(Part {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            link: row.get(2)?,
            exhibit_ids: Vec::new(),
            notes: Vec::new(),
        })
    })?;

    let mut parts = Vec::new();
    for part_res in parts_iter {
        let mut part = part_res?;
        let id = part.id.unwrap();

        // Fetch associated exhibit IDs
        let mut stmt_exhibits = db_conn
            .0
            .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
        let exhibit_ids_iter = stmt_exhibits.query_map(params![id], |row| row.get(0))?;
        part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

        // Fetch associated notes
        let mut stmt_notes = db_conn
            .0
            .prepare("SELECT timestamp, note FROM part_notes WHERE part_id = ?1")?;
        let notes_iter = stmt_notes.query_map(params![id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;
        part.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

        parts.push(part);
    }

    Ok(parts)
}

/// Handles the GET /parts endpoint
#[get("/parts")]
pub async fn list_parts_handler(
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<Vec<crate::models::Part>>, ApiError> {
    let db_conn = db.lock().await;

    match list_parts(&*db_conn) {
        Ok(parts) => Ok(Json(parts)),
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
