use crate::db::DbPool;
use crate::models::{FullName, Jotform, SubmissionDate};
use log::error;
use sqlx::FromRow;
use sqlx::Result;

#[derive(FromRow)]
pub struct JotformRow {
    pub id: String,
    pub submitter_first_name: String,
    pub submitter_last_name: String,
    pub created_at_date: String,
    pub created_at_time: String,
    pub location: String,
    pub exhibit_name: String,
    pub description: String,
    pub priority_level: String,
    pub department: String,
    pub status: String,
}

pub async fn create_jotform_tables(pool: &DbPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS jotforms (
            id TEXT PRIMARY KEY,
            submitter_first_name TEXT NOT NULL,
            submitter_last_name TEXT NOT NULL,
            created_at_date TEXT NOT NULL,
            created_at_time TEXT NOT NULL,
            location TEXT NOT NULL,
            exhibit_name TEXT NOT NULL,
            description TEXT NOT NULL,
            priority_level TEXT NOT NULL,
            department TEXT NOT NULL,
            status TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_jotform(jotform: &Jotform, pool: &DbPool) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO jotforms (
            id,
            submitter_first_name,
            submitter_last_name,
            created_at_date,
            created_at_time,
            location,
            exhibit_name,
            description,
            priority_level,
            department,
            status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(&jotform.id)
    .bind(&jotform.submitter_name.first)
    .bind(&jotform.submitter_name.last)
    .bind(&jotform.created_at.date)
    .bind(&jotform.created_at.time)
    .bind(&jotform.location)
    .bind(&jotform.exhibit_name)
    .bind(&jotform.description)
    .bind(&jotform.priority_level)
    .bind(&jotform.department)
    .bind(&jotform.status)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_jotform(id: String, pool: &DbPool) -> Result<Option<Jotform>> {
    let jotform = sqlx::query_as::<_, JotformRow>("SELECT * FROM jotforms WHERE id = $1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;

    match jotform {
        Some(jotform) => Ok(Some(Jotform {
            id: jotform.id,
            submitter_name: {
                FullName {
                    first: jotform.submitter_first_name,
                    last: jotform.submitter_last_name,
                }
            },
            created_at: {
                SubmissionDate {
                    date: jotform.created_at_date,
                    time: jotform.created_at_time,
                }
            },
            location: jotform.location,
            exhibit_name: jotform.exhibit_name,
            description: jotform.description,
            priority_level: jotform.priority_level,
            department: jotform.department,
            status: jotform.status,
        })),
        None => Ok(None),
    }
}

pub async fn get_all_jotforms(pool: &DbPool) -> Result<Option<Vec<Jotform>>> {
    let jotforms = sqlx::query_as::<_, JotformRow>("SELECT * FROM jotforms")
        .fetch_all(pool)
        .await?;

    if jotforms.is_empty() {
        return Ok(None);
    }

    let jotforms = jotforms
        .iter()
        .map(|jotform| Jotform {
            id: jotform.id.clone(),
            submitter_name: FullName {
                first: jotform.submitter_first_name.clone(),
                last: jotform.submitter_last_name.clone(),
            },
            created_at: {
                SubmissionDate {
                    date: jotform.created_at_date.clone(),
                    time: jotform.created_at_time.clone(),
                }
            },
            location: jotform.location.clone(),
            exhibit_name: jotform.exhibit_name.clone(),
            description: jotform.description.clone(),
            priority_level: jotform.priority_level.clone(),
            department: jotform.department.clone(),
            status: jotform.status.clone(),
        })
        .collect();

    Ok(Some(jotforms))
}

pub async fn update_jotform(jotform: &Jotform, pool: &DbPool) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE jotforms
        SET
            submitter_first_name = $1,
            submitter_last_name = $2,
            created_at_date = $3,
            created_at_time = $4,
            location = $5,
            exhibit_name = $6,
            description = $7,
            priority_level = $8,
            department = $9,
            status = $10
        WHERE id = $11
        "#,
    )
    .bind(&jotform.submitter_name.first)
    .bind(&jotform.submitter_name.last)
    .bind(&jotform.created_at.date)
    .bind(&jotform.created_at.time)
    .bind(&jotform.location)
    .bind(&jotform.exhibit_name)
    .bind(&jotform.description)
    .bind(&jotform.priority_level)
    .bind(&jotform.department)
    .bind(&jotform.status)
    .bind(&jotform.id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_jotform(id: String, pool: &DbPool) -> Result<()> {
    sqlx::query("DELETE FROM jotforms WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}

const STATUS_OPTIONS: [&str; 4] = ["Closed", "InProgress", "Open", "Unplanned"];

pub async fn change_jotform_status(id: String, status: String, pool: &DbPool) -> Result<()> {
    if !STATUS_OPTIONS.contains(&status.as_str()) {
        error!("Invalid status: {}", status);
        return Err(sqlx::Error::RowNotFound.into());
    }

    sqlx::query("UPDATE jotforms SET status = $1 WHERE id = $2")
        .bind(&status)
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn change_jotform_department(
    id: String,
    department: String,
    pool: &DbPool,
) -> Result<()> {
    sqlx::query("UPDATE jotforms SET department = $1 WHERE id = $2")
        .bind(&department)
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn change_jotform_priority(
    id: String,
    priority: String,
    pool: &DbPool,
) -> Result<()> {
    sqlx::query("UPDATE jotforms SET priority_level = $1 WHERE id = $2")
        .bind(&priority)
        .bind(&id)
        .execute(pool)
        .await?;

    Ok(())
}