mod jotform_api;
mod raw_submission;

pub use jotform_api::JotformApi;

use crate::models::Jotform;
use rusqlite::{params, Connection};
use std::collections::HashSet;

pub async fn sync_jotforms_once(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    jotform_api_client: &JotformApi,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1) Get a connection from the pool
    let conn = pool.get()?; // returns a connection, or an error

    // 2) Fetch new submissions from JotForm
    let new_submissions: Vec<Jotform> = jotform_api_client.get_submissions().await?; // returns Vec<Jotform>, or an error

    // 3) Collect existing IDs from local DB
    let mut stmt = conn.prepare("SELECT id FROM jotforms")?;
    let existing_ids: HashSet<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;

    // 4) Insert or update
    for submission in &new_submissions {
        if !existing_ids.contains(&submission.id) {
            insert_jotform(&conn, submission)?;
        } else {
            update_jotform(&conn, submission)?;
        }
    }

    Ok(())
}

fn insert_jotform(conn: &Connection, jotform: &Jotform) -> rusqlite::Result<()> {
    let status = "Open".to_string();

    conn.execute(
        r#"
        INSERT INTO jotforms (
            id, submitter_first_name, submitter_last_name, submission_date, submission_time,
            location, exhibit_name, description,
            priority_level, department, status
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#,
        params![
            jotform.id,
            jotform.submitter_name.first,
            jotform.submitter_name.last,
            jotform.created_at.date,
            jotform.created_at.time,
            jotform.location,
            jotform.exhibit_name,
            jotform.description,
            jotform.priority_level,
            jotform.department,
            status
        ],
    )?;
    Ok(())
}

fn update_jotform(conn: &Connection, jotform: &Jotform) -> rusqlite::Result<()> {
    conn.execute(
        r#"
        UPDATE jotforms
        SET
            submitter_first_name = ?2,
            submitter_last_name = ?3,
            submission_date = ?4,
            submission_time = ?5,
            location = ?6,
            exhibit_name = ?7,
            description = ?8,
            priority_level = ?9,
            department = ?10
        WHERE id = ?1
        "#,
        params![
            jotform.id,
            jotform.submitter_name.first,
            jotform.submitter_name.last,
            jotform.created_at.date,
            jotform.created_at.time,
            jotform.location,
            jotform.exhibit_name,
            jotform.description,
            jotform.priority_level,
            jotform.department,
        ],
    )?;
    Ok(())
}
