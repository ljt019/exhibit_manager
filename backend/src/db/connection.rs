use rusqlite::{Connection, Result as SqliteResult};

pub struct DbConnection(pub Connection);

impl DbConnection {
    /// Initializes a new database connection.
    pub fn new(database_url: &str) -> SqliteResult<Self> {
        let conn = Connection::open(database_url)?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok(DbConnection(conn))
    }

    /// Initializes a new in-memory database connection for testing.
    pub fn new_in_memory() -> SqliteResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok(DbConnection(conn))
    }

    /// Sets up the database schema.
    pub fn setup_tables(&self) -> SqliteResult<()> {
        // Create exhibits table
        self.0.execute(
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
        self.0.execute(
            "CREATE TABLE IF NOT EXISTS parts (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                link TEXT NOT NULL
            )",
            [],
        )?;

        // Create exhibit_parts join table for many-to-many relationship
        self.0.execute(
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
        self.0.execute(
            "CREATE TABLE IF NOT EXISTS exhibit_notes (
                id INTEGER PRIMARY KEY,
                exhibit_id INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                note TEXT NOT NULL,
                FOREIGN KEY (exhibit_id) REFERENCES exhibits(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Create notes table for parts
        self.0.execute(
            "CREATE TABLE IF NOT EXISTS part_notes (
                id INTEGER PRIMARY KEY,
                part_id INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                note TEXT NOT NULL,
                FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    /// Wipes the database by dropping all tables.
    pub fn wipe_database(&self) -> SqliteResult<()> {
        self.0.execute("DROP TABLE IF EXISTS exhibit_parts", [])?;
        self.0.execute("DROP TABLE IF EXISTS exhibit_notes", [])?;
        self.0.execute("DROP TABLE IF EXISTS part_notes", [])?;
        self.0.execute("DROP TABLE IF EXISTS parts", [])?;
        self.0.execute("DROP TABLE IF EXISTS exhibits", [])?;
        Ok(())
    }
}
