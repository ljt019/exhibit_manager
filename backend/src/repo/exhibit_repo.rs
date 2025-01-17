use crate::api::exhibit_handlers::NewExhibit;
use crate::db::DbPool;
use crate::models::{Exhibit, Note, Sponsor, Timestamp, UpdateExhibit};
use sqlx::Result;
use sqlx::Sqlite;

#[derive(sqlx::FromRow)]
struct ExhibitRow {
    id: i64,
    name: String,
    cluster: String,
    location: String,
    description: String,
    status: String,
    image_url: String,
    sponsor_name: Option<String>,
    sponsor_start_date: Option<String>,
    sponsor_end_date: Option<String>,
}

#[derive(sqlx::FromRow)]
struct ExhibitPartRow {
    part_id: i64,
}

#[derive(sqlx::FromRow)]
struct ExhibitNoteRow {
    id: i64,
    date: String,
    time: String,
    message: String,
}

pub async fn create_exhibit_tables(pool: &DbPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS exhibits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            cluster TEXT NOT NULL,
            location TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            image_url TEXT NOT NULL,
            sponsor_name TEXT,
            sponsor_start_date TEXT,
            sponsor_end_date TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

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

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS exhibit_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            exhibit_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            message TEXT NOT NULL,
            FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_exhibit(id: i64, pool: &DbPool) -> Result<Option<Exhibit>> {
    let exhibit_row = sqlx::query_as::<_, ExhibitRow>(
        "SELECT id, name, cluster, location, description, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date 
         FROM exhibits WHERE id = ?1",
    ).bind(id).fetch_optional(pool).await?;

    if let Some(exhibit_row) = exhibit_row {
        let sponsor = match (
            exhibit_row.sponsor_name,
            exhibit_row.sponsor_start_date,
            exhibit_row.sponsor_end_date,
        ) {
            (Some(name), Some(start_date), Some(end_date)) => Some(Sponsor {
                name,
                start_date,
                end_date,
            }),
            _ => None,
        };

        let part_rows = sqlx::query_as::<_, ExhibitPartRow>(
            "SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1",
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        let part_ids = part_rows.iter().map(|row| row.part_id).collect();

        let note_rows = sqlx::query_as::<_, ExhibitNoteRow>(
            "SELECT id, date, time, message FROM exhibit_notes WHERE exhibit_id = ?1",
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        let notes = note_rows
            .iter()
            .map(|row| Note {
                id: row.id,
                timestamp: Timestamp {
                    date: row.date.clone(),
                    time: row.time.clone(),
                },
                message: row.message.clone(),
            })
            .collect();

        Ok(Some(Exhibit {
            id: exhibit_row.id,
            name: exhibit_row.name,
            cluster: exhibit_row.cluster,
            location: exhibit_row.location,
            description: exhibit_row.description,
            status: exhibit_row.status,
            image_url: exhibit_row.image_url,
            sponsor,
            part_ids,
            notes,
        }))
    } else {
        Ok(None)
    }
}

pub async fn create_exhibit(exhibit: &NewExhibit, pool: &DbPool) -> Result<()> {
    let sponsor_name = exhibit.sponsor.as_ref().map(|s| &s.name);
    let sponsor_start = exhibit.sponsor.as_ref().map(|s| &s.start_date);
    let sponsor_end = exhibit.sponsor.as_ref().map(|s| &s.end_date);

    let result = sqlx::query(
        "INSERT INTO exhibits (name, cluster, location, description, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )
    .bind(&exhibit.name)
    .bind(&exhibit.cluster)
    .bind(&exhibit.location)
    .bind(&exhibit.description)
    .bind(&exhibit.status)
    .bind(&exhibit.image_url)
    .bind(sponsor_name)
    .bind(sponsor_start)
    .bind(sponsor_end)
    .execute(pool)
    .await?;

    let exhibit_id = result.last_insert_rowid();

    for part_id in &exhibit.part_ids {
        sqlx::query("INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)")
            .bind(exhibit_id)
            .bind(part_id)
            .execute(pool)
            .await?;
    }

    for note in &exhibit.notes {
        sqlx::query(
            "INSERT INTO exhibit_notes (exhibit_id, date, time, message) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(exhibit_id)
        .bind(&note.timestamp.date)
        .bind(&note.timestamp.time)
        .bind(&note.message)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn update_exhibit(
    id: &i64,
    exhibit: &UpdateExhibit,
    pool: &DbPool,
) -> Result<(), sqlx::Error> {
    // Update the exhibit fields if they are provided
    let mut query = "UPDATE exhibits SET ".to_string();
    let mut params: Vec<String> = Vec::new();

    if let Some(_name) = &exhibit.name {
        params.push("name = ?".to_string());
    }
    if let Some(_cluster) = &exhibit.cluster {
        params.push("cluster = ?".to_string());
    }
    if let Some(_location) = &exhibit.location {
        params.push("location = ?".to_string());
    }
    if let Some(_description) = &exhibit.description {
        params.push("description = ?".to_string());
    }
    if let Some(_image_url) = &exhibit.image_url {
        params.push("image_url = ?".to_string());
    }

    if params.is_empty() {
        // No fields to update
        return Ok(());
    }

    query.push_str(&params.join(", "));
    query.push_str(" WHERE id = ?");

    // Build the query dynamically
    let mut query_builder = sqlx::query::<Sqlite>(&query);

    // Bind values in the correct order
    if let Some(name) = &exhibit.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(cluster) = &exhibit.cluster {
        query_builder = query_builder.bind(cluster);
    }
    if let Some(location) = &exhibit.location {
        query_builder = query_builder.bind(location);
    }
    if let Some(description) = &exhibit.description {
        query_builder = query_builder.bind(description);
    }
    if let Some(image_url) = &exhibit.image_url {
        query_builder = query_builder.bind(image_url);
    }

    // Bind the ID last (for the WHERE clause)
    query_builder = query_builder.bind(id);

    // Execute the query
    query_builder.execute(pool).await?;

    Ok(())
}

pub async fn delete_exhibit(id: i64, pool: &DbPool) -> Result<()> {
    sqlx::query("DELETE FROM exhibits WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_exhibits(pool: &DbPool) -> Result<Option<Vec<Exhibit>>> {
    let exhibit_rows = sqlx::query_as::<_, ExhibitRow>(
        "SELECT id, name, cluster, location, description, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date 
         FROM exhibits",
    )
    .fetch_all(pool)
    .await?;

    let mut exhibits = Vec::new();

    for exhibit_row in exhibit_rows {
        let sponsor = match (
            exhibit_row.sponsor_name,
            exhibit_row.sponsor_start_date,
            exhibit_row.sponsor_end_date,
        ) {
            (Some(name), Some(start_date), Some(end_date)) => Some(Sponsor {
                name,
                start_date,
                end_date,
            }),
            _ => None,
        };

        // Corrected join table name to 'part_exhibits'
        let part_rows = sqlx::query_as::<_, ExhibitPartRow>(
            "SELECT part_id FROM part_exhibits WHERE exhibit_id = ?1",
        )
        .bind(exhibit_row.id)
        .fetch_all(pool)
        .await?;

        let part_ids: Vec<i64> = part_rows.iter().map(|row| row.part_id).collect();

        if part_ids.is_empty() {
            log::info!("Exhibit ID {} has no associated parts.", exhibit_row.id);
        } else {
            log::info!(
                "Exhibit ID {} has {} associated parts.",
                exhibit_row.id,
                part_ids.len()
            );
        }

        let note_rows = sqlx::query_as::<_, ExhibitNoteRow>(
            "SELECT id, date, time, message FROM exhibit_notes WHERE exhibit_id = ?1",
        )
        .bind(exhibit_row.id)
        .fetch_all(pool)
        .await?;

        let notes = note_rows
            .iter()
            .map(|row| Note {
                id: row.id,
                timestamp: Timestamp {
                    date: row.date.clone(),
                    time: row.time.clone(),
                },
                message: row.message.clone(),
            })
            .collect();

        exhibits.push(Exhibit {
            id: exhibit_row.id,
            name: exhibit_row.name,
            cluster: exhibit_row.cluster,
            location: exhibit_row.location,
            description: exhibit_row.description,
            status: exhibit_row.status,
            image_url: exhibit_row.image_url,
            sponsor,
            part_ids,
            notes,
        });
    }

    let response = match exhibits.is_empty() {
        true => None,
        false => Some(exhibits),
    };

    Ok(response)
}

pub async fn get_exhibit_note(id: i64, note_id: i64, pool: &DbPool) -> Result<Option<Note>> {
    let note_row = sqlx::query_as::<_, ExhibitNoteRow>(
        "SELECT id, date, time, message FROM exhibit_notes WHERE exhibit_id = ?1 AND id = ?2",
    )
    .bind(id)
    .bind(note_id)
    .fetch_optional(pool)
    .await?;

    Ok(note_row.map(|row| Note {
        id: row.id,
        timestamp: Timestamp {
            date: row.date,
            time: row.time,
        },
        message: row.message,
    }))
}

pub async fn create_exhibit_note(id: i64, message: String, pool: &DbPool) -> Result<()> {
    sqlx::query(
        "INSERT INTO exhibit_notes (exhibit_id, date, time, message) VALUES (?1, CURRENT_DATE, CURRENT_TIME, ?2)",
    ).bind(id).bind(message).execute(pool).await?;

    Ok(())
}

pub async fn delete_exhibit_note(id: i64, note_id: i64, pool: &DbPool) -> Result<()> {
    sqlx::query("DELETE FROM exhibit_notes WHERE exhibit_id = ?1 AND id = ?2")
        .bind(id)
        .bind(note_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_exhibit_notes(id: i64, pool: &DbPool) -> Result<Option<Vec<Note>>> {
    let note_rows = sqlx::query_as::<_, ExhibitNoteRow>(
        "SELECT id, date, time, message FROM exhibit_notes WHERE exhibit_id = ?1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let notes: Vec<Note> = note_rows
        .iter()
        .map(|row| Note {
            id: row.id,
            timestamp: Timestamp {
                date: row.date.clone(),
                time: row.time.clone(),
            },
            message: row.message.clone(),
        })
        .collect();

    let response = match notes.is_empty() {
        true => None,
        false => Some(notes),
    };

    Ok(response)
}
