// src/db/connection.rs

use crate::repo::{exhibit_repo, jotform_repo, part_repo};
use sqlx::Result as SqlxResult;
use sqlx::SqlitePool;

/// Type alias for the connection pool.
pub type DbPool = SqlitePool;

/// Initializes a new database connection pool.
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    // Check if the database file exists; if not, create it
    if !std::path::Path::new(database_url).exists() {
        let _ = std::fs::File::create(database_url)?;
    }

    SqlitePool::connect(database_url).await
}

/// Sets up the database schema using a connection from the pool.
pub async fn setup_database(pool: &DbPool) -> SqlxResult<()> {
    exhibit_repo::create_exhibit_tables(pool).await?;
    jotform_repo::create_jotform_tables(pool).await?;
    part_repo::create_part_tables(pool).await?;

    Ok(())
}
