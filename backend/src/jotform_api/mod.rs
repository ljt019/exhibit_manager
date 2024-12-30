mod jotform_api;
mod raw_submission;
#[cfg(test)]
mod tests;

pub use jotform_api::JotformApi;
use jotform_api::JotformApiTrait;

use crate::repo::jotform_repo;

use crate::models::Jotform;
use log::info;
use sqlx::SqlitePool;
use std::collections::HashSet;

pub async fn sync_jotforms_once(
    pool: &SqlitePool,
    jotform_api_client: &impl JotformApiTrait,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("! Syncing jotforms !");

    info!("Fetching new submissions from JotForm");
    // 2) Fetch new submissions from JotForm
    let new_submissions: Vec<Jotform> = jotform_api_client.get_submissions().await?; // returns Vec<Jotform>, or an error
    info!("Fetched {} new submissions", new_submissions.len());

    for submission in &new_submissions {
        info!("Submission: {:?}", submission);
    }

    info!("Getting existing IDs from local DB");
    // 3) Collect existing IDs from local DB
    let existing_ids = get_existing_ids(pool).await?;
    info!("Found {} existing IDs", existing_ids.len());
    info!("Existing IDs: {:?}", existing_ids);

    info!("Inserting or updating jotforms");
    // 4) Insert or update
    insert_or_update_jotforms(pool, &new_submissions, &existing_ids).await?;

    info!("! Syncing jotforms complete !");
    Ok(())
}

async fn get_existing_ids(
    pool: &SqlitePool,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let existing_ids = sqlx::query_scalar::<_, String>("SELECT id FROM jotforms")
        .fetch_all(pool)
        .await?;
    Ok(existing_ids.into_iter().collect())
}

async fn insert_or_update_jotforms(
    pool: &SqlitePool,
    new_submissions: &[Jotform],
    existing_ids: &HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for submission in new_submissions {
        info!("Processing submission: {:?}", submission.id);

        if existing_ids.contains(&submission.id) {
            info!("Jotform already existed in the DB, updating");
            jotform_repo::update_jotform(submission, pool).await?;
        } else {
            info!("Jotform didn't exist in the DB, inserting");
            jotform_repo::insert_jotform(submission, pool).await?;
        }
    }
    Ok(())
}
