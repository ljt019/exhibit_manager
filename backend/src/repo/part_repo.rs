use crate::api::parts::create_part::NewPart;
use crate::db::DbPool;
use crate::models::{Note, Part, Timestamp};
use sqlx::Result;

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
    date: String,
    time: String,
    message: String,
}

pub async fn create_part_tables(pool: &DbPool) -> Result<()> {
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

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS part_exhibits (
            part_id INTEGER NOT NULL,
            exhibit_id INTEGER NOT NULL,
            FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE,
            PRIMARY KEY (part_id, exhibit_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS part_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            part_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            message TEXT NOT NULL,
            FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_part(id: i64, pool: &DbPool) -> Result<Option<Part>> {
    let part = sqlx::query_as::<_, PartRow>("SELECT * FROM parts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match part {
        Some(part) => {
            let exhibit_ids = sqlx::query_as::<_, PartExhibitRow>(
                "SELECT exhibit_id FROM part_exhibits WHERE part_id = $1",
            )
            .bind(id)
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| row.exhibit_id)
            .collect();

            let notes =
                sqlx::query_as::<_, PartNoteRow>("SELECT * FROM part_notes WHERE part_id = $1")
                    .bind(id)
                    .fetch_all(pool)
                    .await?
                    .iter()
                    .map(|row| Note {
                        id: row.id,
                        timestamp: {
                            Timestamp {
                                date: row.date.clone(),
                                time: row.time.clone(),
                            }
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
    let result = sqlx::query("INSERT INTO parts (name, link) VALUES ($1, $2)")
        .bind(&part.name)
        .bind(&part.link)
        .execute(pool)
        .await?;

    let part_id = result.last_insert_rowid();

    for exhibit_id in &part.exhibit_ids {
        sqlx::query("INSERT INTO part_exhibits (part_id, exhibit_id) VALUES ($1, $2)")
            .bind(part_id)
            .bind(exhibit_id)
            .execute(pool)
            .await?;
    }

    for note in &part.notes {
        sqlx::query(
            "INSERT INTO part_notes (part_id, date, time, message) VALUES ($1, $2, $3, $4)",
        )
        .bind(part_id)
        .bind(&note.timestamp.date)
        .bind(&note.timestamp.time)
        .bind(&note.message)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn update_part(_id: &i64, part: &Part, pool: &DbPool) -> Result<()> {
    let _result = sqlx::query("UPDATE parts SET name = $1, link = $2 WHERE id = $3")
        .bind(&part.name)
        .bind(&part.link)
        .bind(&part.id)
        .execute(pool)
        .await?;

    sqlx::query("DELETE FROM part_exhibits WHERE part_id = $1")
        .bind(&part.id)
        .execute(pool)
        .await?;

    for exhibit_id in &part.exhibit_ids {
        sqlx::query("INSERT INTO part_exhibits (part_id, exhibit_id) VALUES ($1, $2)")
            .bind(&part.id)
            .bind(exhibit_id)
            .execute(pool)
            .await?;
    }

    sqlx::query("DELETE FROM part_notes WHERE part_id = $1")
        .bind(&part.id)
        .execute(pool)
        .await?;

    for note in &part.notes {
        sqlx::query(
            "INSERT INTO part_notes (part_id, date, time, message) VALUES ($1, $2, $3, $4)",
        )
        .bind(&part.id)
        .bind(&note.timestamp.date)
        .bind(&note.timestamp.time)
        .bind(&note.message)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn delete_part(id: i64, pool: &DbPool) -> Result<()> {
    let _result = sqlx::query("DELETE FROM parts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_parts(pool: &DbPool) -> Result<Option<Vec<Part>>> {
    let parts = sqlx::query_as::<_, PartRow>("SELECT * FROM parts")
        .fetch_all(pool)
        .await?;

    match parts.len() {
        0 => Ok(None),
        _ => {
            let mut part_vec = Vec::new();

            for part in parts {
                let exhibit_ids = sqlx::query_as::<_, PartExhibitRow>(
                    "SELECT exhibit_id FROM part_exhibits WHERE part_id = $1",
                )
                .bind(part.id)
                .fetch_all(pool)
                .await?
                .iter()
                .map(|row| row.exhibit_id)
                .collect();

                let notes =
                    sqlx::query_as::<_, PartNoteRow>("SELECT * FROM part_notes WHERE part_id = $1")
                        .bind(part.id)
                        .fetch_all(pool)
                        .await?
                        .iter()
                        .map(|row| Note {
                            id: row.id,
                            timestamp: {
                                Timestamp {
                                    date: row.date.clone(),
                                    time: row.time.clone(),
                                }
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
    }
}

pub async fn get_parts_by_ids(ids: Vec<i64>, pool: &DbPool) -> Result<Option<Vec<Part>>> {
    let mut part_vec = Vec::new();

    for id in ids {
        let part = sqlx::query_as::<_, PartRow>("SELECT * FROM parts WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        match part {
            Some(part) => {
                let exhibit_ids = sqlx::query_as::<_, PartExhibitRow>(
                    "SELECT exhibit_id FROM part_exhibits WHERE part_id = $1",
                )
                .bind(id)
                .fetch_all(pool)
                .await?
                .iter()
                .map(|row| row.exhibit_id)
                .collect();

                let notes =
                    sqlx::query_as::<_, PartNoteRow>("SELECT * FROM part_notes WHERE part_id = $1")
                        .bind(id)
                        .fetch_all(pool)
                        .await?
                        .iter()
                        .map(|row| Note {
                            id: row.id,
                            timestamp: {
                                Timestamp {
                                    date: row.date.clone(),
                                    time: row.time.clone(),
                                }
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
    let note =
        sqlx::query_as::<_, PartNoteRow>("SELECT * FROM part_notes WHERE part_id = $1 AND id = $2")
            .bind(id)
            .bind(note_id)
            .fetch_optional(pool)
            .await?;

    match note {
        Some(note) => Ok(Some(Note {
            id: note.id,
            timestamp: {
                Timestamp {
                    date: note.date.clone(),
                    time: note.time.clone(),
                }
            },
            message: note.message,
        })),
        None => Ok(None),
    }
}

pub async fn create_part_note(id: i64, message: String, pool: &DbPool) -> Result<()> {
    let _result = sqlx::query(
        "INSERT INTO part_notes (part_id, date, time, message) VALUES ($1, CURRENT_DATE, CURRENT_TIME, $4)",
    )
    .bind(id)
    .bind(&message)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_part_note(id: i64, note_id: i64, pool: &DbPool) -> Result<()> {
    let _result = sqlx::query("DELETE FROM part_notes WHERE part_id = $1 AND id = $2")
        .bind(id)
        .bind(note_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_part_notes(id: i64, pool: &DbPool) -> Result<Option<Vec<Note>>> {
    let notes = sqlx::query_as::<_, PartNoteRow>("SELECT * FROM part_notes WHERE part_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;

    match notes.len() {
        0 => Ok(None),
        _ => {
            let mut note_vec = Vec::new();

            for note in notes {
                note_vec.push(Note {
                    id: note.id,
                    timestamp: {
                        Timestamp {
                            date: note.date.clone(),
                            time: note.time.clone(),
                        }
                    },
                    message: note.message,
                });
            }

            Ok(Some(note_vec))
        }
    }
}
