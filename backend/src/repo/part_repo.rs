use crate::api::part_handlers::NewPart;
use crate::db::DbPool;
use crate::models::{Note, Part, Timestamp, UpdatePart};
use chrono::{DateTime, FixedOffset, Utc};
use sqlx::{Error, Result, Sqlite};

#[derive(sqlx::FromRow)]
struct PartRow {
    id: i64,
    name: String,
    link: String,
}

#[derive(sqlx::FromRow)]
struct PartExhibitRow {
    exhibit_id: i64,
}

#[derive(sqlx::FromRow)]
struct PartNoteRow {
    id: i64,
    submitter: String,
    date: String,
    time: String,
    message: String,
}

pub async fn create_part_tables(pool: &DbPool) -> Result<()> {
    // Create 'parts' table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS parts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            link TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create 'part_notes' table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS part_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            part_id INTEGER NOT NULL,
            submitter TEXT NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            message TEXT NOT NULL,
            FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create 'exhibit_parts' table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS exhibit_parts (
            exhibit_id INTEGER NOT NULL,
            part_id INTEGER NOT NULL,
            FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE,
            PRIMARY KEY (exhibit_id, part_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_part(id: i64, pool: &DbPool) -> Result<Option<Part>> {
    // Use ? placeholders for SQLite
    let part = sqlx::query_as::<_, PartRow>("SELECT id, name, link FROM parts WHERE id = ?1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match part {
        Some(part) => {
            let exhibit_ids = sqlx::query_as::<_, PartExhibitRow>(
                "SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1",
            )
            .bind(id)
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| row.exhibit_id)
            .collect();

            let notes = sqlx::query_as::<_, PartNoteRow>(
                "SELECT id, submitter, date, time, message FROM part_notes WHERE part_id = ?1",
            )
            .bind(id)
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| Note {
                id: row.id,
                submitter: row.submitter.clone(),
                timestamp: Timestamp {
                    date: row.date.clone(),
                    time: row.time.clone(),
                },
                message: row.message.clone(),
            })
            .collect();

            Ok(Some(Part {
                id: Some(part.id),
                name: part.name,
                link: part.link,
                exhibit_ids,
                notes,
            }))
        }
        None => Ok(None),
    }
}

pub async fn create_part(part: &NewPart, pool: &DbPool) -> Result<()> {
    // Insert into 'parts' table
    let result = sqlx::query("INSERT INTO parts (name, link) VALUES (?1, ?2)")
        .bind(&part.name)
        .bind(&part.link)
        .execute(pool)
        .await?;

    let part_id = result.last_insert_rowid();

    // Associate parts with exhibits
    for exhibit_id in &part.exhibit_ids {
        sqlx::query("INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)")
            .bind(exhibit_id) // exhibit_id first
            .bind(part_id)
            .execute(pool)
            .await?;
    }

    // Insert notes with 'submitter'
    for note in &part.notes {
        sqlx::query(
            "INSERT INTO part_notes (part_id, submitter, date, time, message) VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(part_id)
        .bind(&note.submitter) // Bind 'submitter'
        .bind(&note.timestamp.date)
        .bind(&note.timestamp.time)
        .bind(&note.message)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn update_part(id: &i64, part: &UpdatePart, pool: &DbPool) -> Result<()> {
    // Start a transaction since we're making multiple related changes
    let mut tx = pool.begin().await?;

    // Update the main part record
    sqlx::query("UPDATE parts SET name = ?1, link = ?2 WHERE id = ?3")
        .bind(&part.name)
        .bind(&part.link)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    // Delete all existing exhibit associations for this part
    sqlx::query("DELETE FROM exhibit_parts WHERE part_id = ?1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    // Insert new exhibit associations
    for exhibit_id in &part.exhibit_ids {
        sqlx::query("INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)")
            .bind(exhibit_id)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}

pub async fn delete_part(id: i64, pool: &DbPool) -> Result<()> {
    sqlx::query("DELETE FROM parts WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_parts(pool: &DbPool) -> Result<Option<Vec<Part>>> {
    let parts = sqlx::query_as::<_, PartRow>("SELECT id, name, link FROM parts")
        .fetch_all(pool)
        .await?;

    if parts.is_empty() {
        return Ok(None);
    }

    let mut part_vec = Vec::new();

    for part in parts {
        let exhibit_ids = sqlx::query_as::<_, PartExhibitRow>(
            "SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1",
        )
        .bind(part.id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|row| row.exhibit_id)
        .collect();

        let notes = sqlx::query_as::<_, PartNoteRow>(
            "SELECT id, submitter, date, time, message FROM part_notes WHERE part_id = ?1",
        )
        .bind(part.id)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|row| Note {
            id: row.id,
            submitter: row.submitter.clone(),
            timestamp: Timestamp {
                date: row.date.clone(),
                time: row.time.clone(),
            },
            message: row.message.clone(),
        })
        .collect();

        part_vec.push(Part {
            id: Some(part.id),
            name: part.name,
            link: part.link,
            exhibit_ids,
            notes,
        });
    }

    Ok(Some(part_vec))
}

pub async fn get_parts_by_ids(ids: Vec<i64>, pool: &DbPool) -> Result<Option<Vec<Part>>> {
    if ids.is_empty() {
        return Ok(None);
    }

    let mut part_vec = Vec::new();

    for id in ids {
        let part = sqlx::query_as::<_, PartRow>("SELECT id, name, link FROM parts WHERE id = ?1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        match part {
            Some(part) => {
                let exhibit_ids = sqlx::query_as::<_, PartExhibitRow>(
                    "SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1",
                )
                .bind(id)
                .fetch_all(pool)
                .await?
                .iter()
                .map(|row| row.exhibit_id)
                .collect();

                let notes = sqlx::query_as::<_, PartNoteRow>(
                    "SELECT id, submitter, date, time, message FROM part_notes WHERE part_id = ?1",
                )
                .bind(id)
                .fetch_all(pool)
                .await?
                .iter()
                .map(|row| Note {
                    id: row.id,
                    submitter: row.submitter.clone(),
                    timestamp: Timestamp {
                        date: row.date.clone(),
                        time: row.time.clone(),
                    },
                    message: row.message.clone(),
                })
                .collect();

                part_vec.push(Part {
                    id: Some(part.id),
                    name: part.name,
                    link: part.link,
                    exhibit_ids,
                    notes,
                });
            }
            None => return Ok(None),
        }
    }

    Ok(Some(part_vec))
}

pub async fn get_part_note(id: i64, note_id: i64, pool: &DbPool) -> Result<Option<Note>> {
    let note = sqlx::query_as::<_, PartNoteRow>(
        "SELECT id, submitter, date, time, message FROM part_notes WHERE part_id = ?1 AND id = ?2",
    )
    .bind(id)
    .bind(note_id)
    .fetch_optional(pool)
    .await?;

    match note {
        Some(note) => Ok(Some(Note {
            id: note.id,
            submitter: note.submitter,
            timestamp: Timestamp {
                date: note.date,
                time: note.time,
            },
            message: note.message,
        })),
        None => Ok(None),
    }
}

pub async fn create_part_note(
    id: i64,
    submitter: String,
    message: String,
    pool: &DbPool,
) -> Result<()> {
    // Get the current time in UTC
    let now_utc: DateTime<Utc> = Utc::now();

    // Convert the time to Central Time (UTC-6)
    let now_central = now_utc.with_timezone(&FixedOffset::west_opt(6 * 3600).unwrap());

    // Extract the date and time components
    let date = now_central.date_naive().to_string();
    let time = now_central.time().to_string();

    sqlx::query(
        "INSERT INTO part_notes (part_id, submitter, date, time, message) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(id)
    .bind(submitter)
    .bind(date)
    .bind(time)
    .bind(message)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_part_note(id: i64, note_id: i64, pool: &DbPool) -> Result<()> {
    sqlx::query("DELETE FROM part_notes WHERE part_id = ?1 AND id = ?2")
        .bind(id)
        .bind(note_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_part_notes(id: i64, pool: &DbPool) -> Result<Option<Vec<Note>>> {
    let notes = sqlx::query_as::<_, PartNoteRow>(
        "SELECT id, submitter, date, time, message FROM part_notes WHERE part_id = ?1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    if notes.is_empty() {
        return Ok(None);
    }

    let note_vec = notes
        .iter()
        .map(|note| Note {
            id: note.id,
            submitter: note.submitter.clone(),
            timestamp: Timestamp {
                date: note.date.clone(),
                time: note.time.clone(),
            },
            message: note.message.clone(),
        })
        .collect();

    Ok(Some(note_vec))
}
