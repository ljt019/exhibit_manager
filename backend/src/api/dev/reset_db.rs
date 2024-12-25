use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::get;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;
use rusqlite::Result as SqliteResult;

/// Resets the database by wiping all data and setting up tables.
///
/// This function performs a complete reset of the database by removing all existing data
/// and reinitializing the necessary tables.
///
/// # Arguments
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<()>` - Returns `Ok(())` if the reset is successful.
/// * `rusqlite::Error` - Returns an error if any database operation fails.
pub fn reset_database(conn: &Connection) -> rusqlite::Result<()> {
    wipe_database(conn)?;
    setup_tables(conn)?;

    Ok(())
}

/// Handles the GET /reset endpoint.
///
/// This endpoint resets the database by wiping all existing data and setting up the necessary tables.
/// It returns a success message upon completion.
///
/// # Arguments
/// * `db_pool` - A reference to the database connection pool.
///
/// # Returns
/// * `Result<Json<serde_json::Value>, ApiError>` - Returns a JSON message indicating success or an error.
///
/// # Errors
/// Returns an `ApiError` if:
/// - The database connection cannot be obtained.
/// - A database operation fails.
#[get("/reset")]
pub async fn handle_reset_db(db_pool: &State<DbPool>) -> Result<Json<serde_json::Value>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let _ = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        reset_database(&conn).map_err(|e| {
            error!("Failed to reset database: {}", e);
            ApiError::DatabaseError("Failed to reset database".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked while resetting database: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(serde_json::json!({
        "message": "Database reset successful"
    })))
}

/// Sets up the database schema.
fn setup_tables(conn: &Connection) -> SqliteResult<()> {
    // Create exhibits table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exhibits (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                cluster TEXT NOT NULL,
                location TEXT NOT NULL,
                status TEXT NOT NULL,
                image_url TEXT NOT NULL,
                sponsor_name TEXT,
                sponsor_start_date TEXT,
                sponsor_end_date TEXT
            )",
        [],
    )?;

    // Create parts table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS parts (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                link TEXT NOT NULL
            )",
        [],
    )?;

    // Create exhibit_parts join table for many-to-many relationship
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exhibit_parts (
                exhibit_id INTEGER NOT NULL,
                part_id INTEGER NOT NULL,
                FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE,
                FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE,
                PRIMARY KEY (exhibit_id, part_id)
            )",
        [],
    )?;

    // Create notes table for exhibits
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exhibit_notes (
                id INTEGER PRIMARY KEY,
                exhibit_id INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                message TEXT NOT NULL,
                FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE
            )",
        [],
    )?;

    // Create notes table for parts
    conn.execute(
        "CREATE TABLE IF NOT EXISTS part_notes (
                id INTEGER PRIMARY KEY,
                part_id INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                message TEXT NOT NULL,
                FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE
            )",
        [],
    )?;

    Ok(())
}

/// Wipes the database by dropping all tables.
fn wipe_database(conn: &Connection) -> SqliteResult<()> {
    conn.execute("DROP TABLE IF EXISTS exhibit_parts", [])?;
    conn.execute("DROP TABLE IF EXISTS exhibit_notes", [])?;
    conn.execute("DROP TABLE IF EXISTS part_notes", [])?;
    conn.execute("DROP TABLE IF EXISTS parts", [])?;
    conn.execute("DROP TABLE IF EXISTS exhibits", [])?;
    Ok(())
}
