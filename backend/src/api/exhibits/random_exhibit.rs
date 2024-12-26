use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::{Exhibit, Note, Sponsor, Timestamp};
use log::error;
use rand::seq::SliceRandom;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Fetches a single random exhibit from the database.
pub fn random_exhibit(conn: &Connection) -> rusqlite::Result<Exhibit> {
    let mut stmt = conn.prepare(
        "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date FROM exhibits"
    )?;

    let exhibits_iter = stmt.query_map([], |row| {
        let sponsor = match (
            row.get::<_, Option<String>>(6)?,
            row.get::<_, Option<String>>(7)?,
            row.get::<_, Option<String>>(8)?,
        ) {
            (Some(name), Some(start_date), Some(end_date)) => Some(Sponsor {
                name,
                start_date,
                end_date,
            }),
            _ => None,
        };

        Ok(Exhibit {
            id: row.get(0)?,
            name: row.get(1)?,
            cluster: row.get(2)?,
            location: row.get(3)?,
            status: row.get(4)?,
            image_url: row.get(5)?,
            sponsor,
            part_ids: Vec::new(),
            notes: Vec::new(),
        })
    })?;

    let mut exhibits = Vec::new();
    for exhibit_res in exhibits_iter {
        let mut exhibit = exhibit_res?;
        let id = exhibit.id;

        let mut stmt_parts =
            conn.prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
        let part_ids_iter = stmt_parts.query_map(rusqlite::params![id], |row| row.get(0))?;
        exhibit.part_ids = part_ids_iter.collect::<rusqlite::Result<Vec<i64>>>()?;

        // Updated to handle new Timestamp structure
        let mut stmt_notes = conn
            .prepare("SELECT id, date, time, message FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt_notes.query_map(rusqlite::params![id], |row| {
            Ok(Note {
                id: row.get(0)?,
                timestamp: Timestamp {
                    date: row.get(1)?,
                    time: row.get(2)?,
                },
                message: row.get(3)?,
            })
        })?;
        exhibit.notes = notes_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

        exhibits.push(exhibit);
    }

    if exhibits.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let random_exhibit = exhibits.choose(&mut rand::thread_rng()).unwrap().clone();
    Ok(random_exhibit)
}

#[get("/exhibits/random")]
pub async fn handle_random_exhibit(db_pool: &State<DbPool>) -> Result<Json<Exhibit>, ApiError> {
    let pool = (*db_pool).clone();

    let result = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        random_exhibit(&conn).map_err(|e| {
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
