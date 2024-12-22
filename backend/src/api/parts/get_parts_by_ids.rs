use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Note;
use crate::models::Part;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::{params, Result as SqliteResult};

fn get_parts_by_ids(ids: &[i64], db_conn: &DbConnection) -> SqliteResult<Vec<Part>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    // Create a string of placeholders (?, ?, ?, ...)
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query = format!(
        "SELECT id, name, link FROM parts WHERE id IN ({})",
        placeholders
    );

    let mut stmt = db_conn.0.prepare(&query)?;

    // Convert ids to a vector of references for parameter binding
    let id_refs: Vec<&dyn rusqlite::ToSql> =
        ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

    let parts_iter = stmt.query_map(&*id_refs, |row| {
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

/// Handles the POST /parts/batch endpoint
#[post("/parts/batch", format = "json", data = "<part_ids>")]
pub async fn get_parts_by_ids_handler(
    part_ids: Json<Vec<i64>>,
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<Vec<crate::models::Part>>, ApiError> {
    let part_ids = part_ids.into_inner();
    info!("Received /parts/batch request with IDs: {:?}", part_ids);

    if part_ids.is_empty() {
        info!("Empty part_ids received.");
        return Err(ApiError::InvalidRequestBody);
    }

    let db_conn = db.lock().await;

    match get_parts_by_ids(&part_ids, &*db_conn) {
        Ok(parts) => {
            info!("Successfully retrieved {} parts.", parts.len());
            Ok(Json(parts))
        }
        Err(e) => {
            error!("Database error while fetching parts: {:?}", e);
            Err(ApiError::DatabaseError("Database Error".to_string()))
        }
    }
}
