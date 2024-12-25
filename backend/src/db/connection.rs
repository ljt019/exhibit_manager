// src/db/connection.rs

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result as SqliteResult;

/// Type alias for the connection pool.
pub type DbPool = Pool<SqliteConnectionManager>;

/// Initializes a new database connection pool.
pub fn create_pool(database_url: &str) -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file(database_url);
    Pool::new(manager)
}

/// Sets up the database schema using a connection from the pool.
pub fn setup_database(pool: &DbPool) -> SqliteResult<()> {
    let conn = pool.get().expect("Failed to get DB connection from pool");
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS exhibits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            cluster TEXT NOT NULL,
            location TEXT NOT NULL,
            status TEXT NOT NULL,
            image_url TEXT NOT NULL,
            sponsor_name TEXT,
            sponsor_start_date TEXT,
            sponsor_end_date TEXT
        );

        CREATE TABLE IF NOT EXISTS parts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            link TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS exhibit_parts (
            exhibit_id INTEGER NOT NULL,
            part_id INTEGER NOT NULL,
            FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE,
            FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE,
            PRIMARY KEY (exhibit_id, part_id)
        );

        CREATE TABLE IF NOT EXISTS exhibit_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            exhibit_id INTEGER NOT NULL,
            timestamp TEXT NOT NULL,
            message TEXT NOT NULL,
            FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS part_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            part_id INTEGER NOT NULL,
            timestamp TEXT NOT NULL,
            message TEXT NOT NULL,
            FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS jotforms (
            id TEXT PRIMARY KEY,
            submitter_name TEXT NOT NULL,
            submission_date TEXT NOT NULL,
            submission_time TEXT NOT NULL,
            location TEXT NOT NULL,
            exhibit_name TEXT NOT NULL,
            description TEXT NOT NULL,
            priority_level TEXT NOT NULL,
            department TEXT NOT NULL,
            status TEXT NOT NULL
        );
        ",
    )
}
