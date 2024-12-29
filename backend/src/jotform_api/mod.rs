mod jotform_api;
mod raw_submission;

pub use jotform_api::JotformApi;

use crate::models::Jotform;
use log::info;
use rusqlite::{params, Connection};
use std::collections::HashSet;

pub async fn sync_jotforms_once(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    jotform_api_client: &JotformApi,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FullName;
    use crate::models::Jotform;
    use crate::models::SubmissionDate;
    use rusqlite::params;

    struct TestJotformData {
        old_jotforms: Vec<Jotform>,
        new_jotforms: Vec<Jotform>,
    }

    fn get_fake_jotforms() -> TestJotformData {
        let old_jotforms = vec![
            Jotform {
                id: "6091467635319479371".to_string(),
                submitter_name: FullName {
                    first: "Kenneth".to_string(),
                    last: "Smith".to_string(),
                },
                created_at: SubmissionDate {
                    date: "2024-12-04".to_string(),
                    time: "13:39:24".to_string(),
                },
                location: "Deep Space".to_string(),
                exhibit_name: "Electromagnetic Spectrum".to_string(),
                description: "It's peeling off the wall.".to_string(),
                priority_level: "High".to_string(),
                department: "Operations".to_string(),
                status: "Open".to_string(),
            },
            Jotform {
                id: "6081117525314833207".to_string(),
                submitter_name: FullName {
                    first: "Kenneth".to_string(),
                    last: "Smith".to_string(),
                },
                created_at: SubmissionDate {
                    date: "2024-11-22".to_string(),
                    time: "14:09:13".to_string(),
                },
                location: "Space".to_string(),
                exhibit_name: "Moon Chair".to_string(),
                description: "One of the moons is cracked".to_string(),
                priority_level: "High".to_string(),
                department: "Exhibits".to_string(),
                status: "Open".to_string(),
            },
        ];

        let new_jotforms = vec![
            Jotform {
                id: "6111451635317428145".to_string(),
                submitter_name: FullName {
                    first: "Lou".to_string(),
                    last: "Papai".to_string(),
                },
                created_at: SubmissionDate {
                    date: "2024-12-27".to_string(),
                    time: "16:46:03".to_string(),
                },
                location: "Solarium".to_string(),
                exhibit_name: "Solarium Signage".to_string(),
                description: "The sign for the Solarium needs re-mounted on the wall - maybe above the bridge? It explains the 3 parts of the Solarium.".to_string(),
                priority_level: "High".to_string(),
                department: "Exhibits".to_string(),
                status: "Open".to_string(),
            },
            Jotform {
                id: "6111430635314685470".to_string(),
                submitter_name: FullName {
                    first: "Lou".to_string(),
                    last: "Papai".to_string(),
                },
                created_at: SubmissionDate {
                    date: "2024-12-27".to_string(),
                    time: "16:11:03".to_string(),
                },
                location: "PoP Children's Museum".to_string(),
                exhibit_name: "Water Table".to_string(),
                description: "The PoP Water Table’s water is low at the end in the large circle. I think maybe the filter might need cleaned.".to_string(),
                priority_level: "High".to_string(),
                department: "Exhibits".to_string(),
                status: "Open".to_string(),
            },
        ];

        TestJotformData {
            old_jotforms,
            new_jotforms,
        }
    }

    fn setup_test_db() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE jotforms (
                id TEXT PRIMARY KEY,
                submitter_first_name TEXT NOT NULL,
                submitter_last_name TEXT NOT NULL,
                submission_date TEXT NOT NULL,
                submission_time TEXT NOT NULL,
                location TEXT NOT NULL,
                exhibit_name TEXT NOT NULL,
                description TEXT NOT NULL,
                priority_level TEXT NOT NULL,
                department TEXT NOT NULL,
                status TEXT NOT NULL
            )
            "#,
            [],
        )
        .unwrap();

        let test_data = get_fake_jotforms();

        // add old jotforms to the test db
        for jotform in test_data.old_jotforms {
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
                    jotform.status
                ],
            )
            .unwrap();
        }

        conn
    }

    #[test]
    fn test_update_jotforms() -> Result<(), Box<dyn std::error::Error>> {
        let jotforms_test_data = get_fake_jotforms();
        let conn = setup_test_db();

        let existing_ids: HashSet<String> = jotforms_test_data
            .old_jotforms
            .iter()
            .map(|j| j.id.clone())
            .collect();

        let final_ids: HashSet<String> = jotforms_test_data
            .old_jotforms
            .iter()
            .map(|j| j.id.clone())
            .chain(jotforms_test_data.new_jotforms.iter().map(|j| j.id.clone()))
            .collect();

        // Process new submissions
        for submission in &jotforms_test_data.new_jotforms {
            if !existing_ids.contains(&submission.id) {
                insert_jotform(&conn, submission)?;
            } else {
                update_jotform(&conn, submission)?;
            }
        }

        // Verify results
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM jotforms")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;

        assert_eq!(
            count as usize,
            jotforms_test_data.old_jotforms.len() + jotforms_test_data.new_jotforms.len()
        );

        Ok(())
    }
}
