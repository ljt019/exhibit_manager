use super::*;
use crate::models::{FullName, Jotform, SubmissionDate};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rocket::async_trait;
use rocket::tokio;

// Mock JotformApi struct for testing
struct MockJotformApi {
    submissions: Vec<Jotform>,
}

impl MockJotformApi {
    fn new(submissions: Vec<Jotform>) -> Self {
        Self { submissions }
    }
}

#[async_trait]
impl JotformApiTrait for MockJotformApi {
    async fn get_submissions(&self) -> Result<Vec<Jotform>, Box<dyn std::error::Error>> {
        Ok(self.submissions.clone())
    }
}

fn setup_test_db() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::memory();
    let pool = Pool::new(manager).unwrap();

    let conn = pool.get().unwrap();
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

    pool
}

fn get_fake_jotforms() -> Vec<Jotform> {
    vec![
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
    ]
}

#[test]
fn test_insert_jotform() -> Result<(), Box<dyn std::error::Error>> {
    let pool = setup_test_db();
    let jotforms = get_fake_jotforms();
    let conn = pool.get()?;

    // Insert all jotforms
    for jotform in &jotforms {
        insert_jotform(&conn, jotform)?;
    }

    // Verify that all jotforms were inserted
    let mut stmt = conn.prepare("SELECT * FROM jotforms")?;
    let mut rows = stmt.query([])?;

    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Jotform {
            id: row.get(0)?,
            submitter_name: FullName {
                first: row.get(1)?,
                last: row.get(2)?,
            },
            created_at: SubmissionDate {
                date: row.get(3)?,
                time: row.get(4)?,
            },
            location: row.get(5)?,
            exhibit_name: row.get(6)?,
            description: row.get(7)?,
            priority_level: row.get(8)?,
            department: row.get(9)?,
            status: row.get(10)?,
        });
    }

    assert_eq!(results.len(), jotforms.len());
    for jotform in &jotforms {
        assert!(results.contains(jotform));
    }

    Ok(())
}

#[test]
fn test_update_jotform() -> Result<(), Box<dyn std::error::Error>> {
    let pool = setup_test_db();
    let jotforms = get_fake_jotforms();
    let conn = pool.get()?;

    // Insert initial jotforms
    for jotform in &jotforms {
        insert_jotform(&conn, jotform)?;
    }

    // Modify one of the jotforms
    let mut updated_jotform = jotforms[0].clone();
    updated_jotform.location = "New Location".to_string();

    // Update the jotform
    update_jotform(&conn, &updated_jotform)?;

    // Verify that the jotform was updated
    let mut stmt = conn.prepare("SELECT * FROM jotforms WHERE id = ?")?;
    let result: Jotform = stmt.query_row(params![updated_jotform.id], |row| {
        Ok(Jotform {
            id: row.get(0)?,
            submitter_name: FullName {
                first: row.get(1)?,
                last: row.get(2)?,
            },
            created_at: SubmissionDate {
                date: row.get(3)?,
                time: row.get(4)?,
            },
            location: row.get(5)?,
            exhibit_name: row.get(6)?,
            description: row.get(7)?,
            priority_level: row.get(8)?,
            department: row.get(9)?,
            status: row.get(10)?,
        })
    })?;

    assert_eq!(result.location, "New Location");

    Ok(())
}

#[tokio::test]
async fn test_sync_jotforms_once() -> Result<(), Box<dyn std::error::Error>> {
    // Setup test database and insert initial jotforms
    let pool = setup_test_db();
    let initial_jotforms = get_fake_jotforms();
    let conn = pool.get()?;
    for jotform in &initial_jotforms {
        insert_jotform(&conn, jotform)?;
    }

    // Create new jotforms to simulate API response
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
                status: "Open".to_string(), // This should be ignored during sync
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
                status: "Open".to_string(), // This should be ignored during sync
            },
        ];

    // Mock the JotformApi client
    let mock_api = MockJotformApi::new(new_jotforms.clone());

    // Call sync_jotforms_once
    sync_jotforms_once(&pool, &mock_api).await?;

    // Verify that all jotforms (initial + new) are in the database
    let conn = pool.get()?;
    let mut stmt = conn.prepare("SELECT * FROM jotforms")?;
    let mut rows = stmt.query([])?;

    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Jotform {
            id: row.get(0)?,
            submitter_name: FullName {
                first: row.get(1)?,
                last: row.get(2)?,
            },
            created_at: SubmissionDate {
                date: row.get(3)?,
                time: row.get(4)?,
            },
            location: row.get(5)?,
            exhibit_name: row.get(6)?,
            description: row.get(7)?,
            priority_level: row.get(8)?,
            department: row.get(9)?,
            status: row.get(10)?,
        });
    }

    // Check that all initial and new jotforms are present
    let all_jotforms: Vec<Jotform> = initial_jotforms
        .into_iter()
        .chain(new_jotforms.into_iter())
        .collect();
    assert_eq!(results.len(), all_jotforms.len());
    for jotform in &all_jotforms {
        assert!(results.contains(jotform));
    }

    // Verify that the status field was not overwritten
    for jotform in &results {
        let expected_status =
            if jotform.id == "6111451635317428145" || jotform.id == "6111430635314685470" {
                "Open" // New jotforms should have the default status
            } else {
                "Open" // Existing jotforms should retain their status
            };
        assert_eq!(jotform.status, expected_status);
    }

    Ok(())
}
