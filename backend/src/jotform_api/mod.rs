mod jotform_api;
mod raw_submission;
#[cfg(test)]
mod tests;

pub use jotform_api::JotformApi;
use jotform_api::JotformApiTrait;

use crate::models::Jotform;
use log::info;
use rusqlite::{params, Connection};
use std::collections::HashSet;

pub async fn sync_jotforms_once(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    jotform_api_client: &impl JotformApiTrait,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("! Syncing jotforms !");

    info!("Getting a connection from the pool");
    // 1) Get a connection from the pool
    let conn = pool.get()?; // returns a connection, or an error

    info!("Fetching new submissions from JotForm");
    // 2) Fetch new submissions from JotForm
    let new_submissions: Vec<Jotform> = jotform_api_client.get_submissions().await?; // returns Vec<Jotform>, or an error
    info!("Fetched {} new submissions", new_submissions.len());

    for submission in &new_submissions {
        info!("Submission: {:?}", submission);
    }

    info!("Getting existing IDs from local DB");
    // 3) Collect existing IDs from local DB
    let existing_ids = get_existing_ids(&conn)?;
    info!("Found {} existing IDs", existing_ids.len());
    info!("Existing IDs: {:?}", existing_ids);

    info!("Inserting or updating jotforms");
    // 4) Insert or update
    insert_or_update_jotforms(&conn, &new_submissions, &existing_ids)?;

    info!("! Syncing jotforms complete !");
    Ok(())
}

fn get_existing_ids(
    conn: &r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id FROM jotforms")?;
    let existing_ids: HashSet<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;
    Ok(existing_ids)
}

fn insert_or_update_jotforms(
    conn: &r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>,
    new_submissions: &[Jotform],
    existing_ids: &HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for submission in new_submissions {
        info!("Processing submission: {:?}", submission.id);

        if existing_ids.contains(&submission.id) {
            info!("Jotform already existed in the DB, updating");
            update_jotform(conn, submission)?;
        } else {
            info!("Jotform didn't exist in the DB, inserting");
            insert_jotform(conn, submission)?;
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
