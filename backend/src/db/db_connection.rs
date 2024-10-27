use crate::models::{Exhibit, Note, Part};
use chrono::{Duration, Utc};
use rand::Rng;
use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Result as SqliteResult};

pub struct DbConnection(Connection);

impl DbConnection {
    /// Initializes a new database connection and sets up the necessary tables.
    pub fn new() -> SqliteResult<Self> {
        let conn = Connection::open("exhibits.db")?;

        // Enable foreign key support
        conn.execute("PRAGMA foreign_keys = ON;", [])?;

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
                note TEXT NOT NULL,
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
                note TEXT NOT NULL,
                FOREIGN KEY (part_id) REFERENCES parts(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(DbConnection(conn))
    }

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

    /// Creates a new exhibit in the database.
    pub fn create_exhibit(&self, exhibit: &Exhibit) -> SqliteResult<i64> {
        self.0.execute(
            "INSERT INTO exhibits (name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                exhibit.name,
                exhibit.cluster,
                exhibit.location,
                exhibit.status,
                exhibit.image_url,
                exhibit.sponsor_name,
                exhibit.sponsor_start_date,
                exhibit.sponsor_end_date,
            ],
        )?;
        let exhibit_id = self.0.last_insert_rowid();

        // Associate parts with the exhibit
        for part_id in &exhibit.part_ids {
            self.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![exhibit_id, part_id],
            )?;
        }

        // Insert notes related to the exhibit
        for note in &exhibit.notes {
            self.0.execute(
                "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![exhibit_id, &note.timestamp, &note.note],
            )?;
        }

        Ok(exhibit_id)
    }

    /// Retrieves an exhibit by its ID, including associated parts and notes.
    pub fn get_exhibit(&self, id: i64) -> SqliteResult<Option<Exhibit>> {
        let exhibit_opt = self.0
            .query_row(
                "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date 
                 FROM exhibits WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Exhibit {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        cluster: row.get(2)?,
                        location: row.get(3)?,
                        status: row.get(4)?,
                        image_url: row.get(5)?,
                        sponsor_name: row.get(6)?,
                        sponsor_start_date: row.get(7)?,
                        sponsor_end_date: row.get(8)?,
                        part_ids: Vec::new(), // To be populated
                        notes: Vec::new(),    // To be populated
                    })
                },
            )
            .optional()?;

        if let Some(mut exhibit) = exhibit_opt {
            // Fetch associated part IDs
            let mut stmt = self
                .0
                .prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
            let part_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
            exhibit.part_ids = part_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt = self
                .0
                .prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
            let notes_iter = stmt.query_map(params![id], |row| {
                Ok(Note {
                    timestamp: row.get(0)?,
                    note: row.get(1)?,
                })
            })?;
            exhibit.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

            Ok(Some(exhibit))
        } else {
            Ok(None)
        }
    }

    /// Updates an existing exhibit's details, including its associated parts and notes.
    pub fn update_exhibit(&self, id: i64, exhibit: &Exhibit) -> SqliteResult<usize> {
        let affected = self.0.execute(
            "UPDATE exhibits 
             SET name = ?1, cluster = ?2, location = ?3, status = ?4, image_url = ?5, 
                 sponsor_name = ?6, sponsor_start_date = ?7, sponsor_end_date = ?8 
             WHERE id = ?9",
            params![
                exhibit.name,
                exhibit.cluster,
                exhibit.location,
                exhibit.status,
                exhibit.image_url,
                exhibit.sponsor_name,
                exhibit.sponsor_start_date,
                exhibit.sponsor_end_date,
                id
            ],
        )?;

        // Update associated parts: Remove existing and add new associations
        self.0.execute(
            "DELETE FROM exhibit_parts WHERE exhibit_id = ?1",
            params![id],
        )?;
        for part_id in &exhibit.part_ids {
            self.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![id, part_id],
            )?;
        }

        // Update notes: Remove existing and add new notes
        self.0.execute(
            "DELETE FROM exhibit_notes WHERE exhibit_id = ?1",
            params![id],
        )?;
        for note in &exhibit.notes {
            self.0.execute(
                "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![id, &note.timestamp, &note.note],
            )?;
        }

        Ok(affected)
    }

    /// Deletes an exhibit by its ID. Associated `exhibit_parts` and `exhibit_notes` are automatically deleted due to cascading.
    pub fn delete_exhibit(&self, id: i64) -> SqliteResult<usize> {
        self.0
            .execute("DELETE FROM exhibits WHERE id = ?1", params![id])
    }

    /// Lists all exhibits, including their associated parts and notes.
    pub fn list_exhibits(&self) -> SqliteResult<Vec<Exhibit>> {
        let mut stmt = self.0.prepare(
            "SELECT id, name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date FROM exhibits"
        )?;
        let exhibits_iter = stmt.query_map([], |row| {
            Ok(Exhibit {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                cluster: row.get(2)?,
                location: row.get(3)?,
                status: row.get(4)?,
                image_url: row.get(5)?,
                sponsor_name: row.get(6)?,
                sponsor_start_date: row.get(7)?,
                sponsor_end_date: row.get(8)?,
                part_ids: Vec::new(), // To be populated
                notes: Vec::new(),    // To be populated
            })
        })?;

        let mut exhibits = Vec::new();
        for exhibit_res in exhibits_iter {
            let mut exhibit = exhibit_res?;
            let id = exhibit.id.unwrap();

            // Fetch associated part IDs
            let mut stmt_parts = self
                .0
                .prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
            let part_ids_iter = stmt_parts.query_map(params![id], |row| row.get(0))?;
            exhibit.part_ids = part_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt_notes = self
                .0
                .prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
            let notes_iter = stmt_notes.query_map(params![id], |row| {
                Ok(Note {
                    timestamp: row.get(0)?,
                    note: row.get(1)?,
                })
            })?;
            exhibit.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

            exhibits.push(exhibit);
        }

        Ok(exhibits)
    }

    /// Creates a new part in the database.
    pub fn create_part(&self, part: &Part) -> SqliteResult<i64> {
        self.0.execute(
            "INSERT INTO parts (name, link) VALUES (?1, ?2)",
            params![part.name, part.link],
        )?;
        let part_id = self.0.last_insert_rowid();

        // Associate exhibits with the part
        for exhibit_id in &part.exhibit_ids {
            self.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![exhibit_id, part_id],
            )?;
        }

        // Insert notes related to the part
        for note in &part.notes {
            self.0.execute(
                "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![part_id, &note.timestamp, &note.note],
            )?;
        }

        Ok(part_id)
    }

    /// Retrieves a part by its ID, including associated exhibits and notes.
    pub fn get_part(&self, id: i64) -> SqliteResult<Option<Part>> {
        let part_opt = self
            .0
            .query_row(
                "SELECT id, name, link FROM parts WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Part {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        link: row.get(2)?,
                        exhibit_ids: Vec::new(), // To be populated
                        notes: Vec::new(),       // To be populated
                    })
                },
            )
            .optional()?;

        if let Some(mut part) = part_opt {
            // Fetch associated exhibit IDs
            let mut stmt = self
                .0
                .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
            let exhibit_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
            part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt = self
                .0
                .prepare("SELECT timestamp, note FROM part_notes WHERE part_id = ?1")?;
            let notes_iter = stmt.query_map(params![id], |row| {
                Ok(Note {
                    timestamp: row.get(0)?,
                    note: row.get(1)?,
                })
            })?;
            part.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

            Ok(Some(part))
        } else {
            Ok(None)
        }
    }

    /// Updates an existing part's details, including its associated exhibits and notes.
    pub fn update_part(&self, id: i64, part: &Part) -> SqliteResult<usize> {
        let affected = self.0.execute(
            "UPDATE parts SET name = ?1, link = ?2 WHERE id = ?3",
            params![part.name, part.link, id],
        )?;

        // Update associated exhibits: Remove existing and add new associations
        self.0
            .execute("DELETE FROM exhibit_parts WHERE part_id = ?1", params![id])?;
        for exhibit_id in &part.exhibit_ids {
            self.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![exhibit_id, id],
            )?;
        }

        // Update notes: Remove existing and add new notes
        self.0
            .execute("DELETE FROM part_notes WHERE part_id = ?1", params![id])?;
        for note in &part.notes {
            self.0.execute(
                "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![id, &note.timestamp, &note.note],
            )?;
        }

        Ok(affected)
    }

    /// Deletes a part by its ID. Associated `exhibit_parts` and `part_notes` are automatically deleted due to cascading.
    pub fn delete_part(&self, id: i64) -> SqliteResult<usize> {
        self.0
            .execute("DELETE FROM parts WHERE id = ?1", params![id])
    }

    /// Lists all parts, including their associated exhibits and notes.
    pub fn list_parts(&self) -> SqliteResult<Vec<Part>> {
        let mut stmt = self.0.prepare("SELECT id, name, link FROM parts")?;
        let parts_iter = stmt.query_map([], |row| {
            Ok(Part {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                link: row.get(2)?,
                exhibit_ids: Vec::new(), // To be populated
                notes: Vec::new(),       // To be populated
            })
        })?;

        let mut parts = Vec::new();
        for part_res in parts_iter {
            let mut part = part_res?;
            let id = part.id.unwrap();

            // Fetch associated exhibit IDs
            let mut stmt_exhibits = self
                .0
                .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
            let exhibit_ids_iter = stmt_exhibits.query_map(params![id], |row| row.get(0))?;
            part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt_notes = self
                .0
                .prepare("SELECT timestamp, note FROM part_notes WHERE part_id = ?1")?;
            let notes_iter = stmt_notes.query_map(params![id], |row| {
                Ok(Note {
                    timestamp: row.get(0)?,
                    note: row.get(1)?,
                })
            })?;
            part.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

            parts.push(part);
        }

        Ok(parts)
    }

    pub fn get_parts_by_ids(&self, ids: &[i64]) -> SqliteResult<Vec<Part>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        // Prepare the SQL query with the appropriate number of placeholders
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            "SELECT id, name, link FROM parts WHERE id IN ({})",
            placeholders
        );

        let mut stmt = self.0.prepare(&query)?;

        // Convert ids to a vector of references for parameter binding
        let id_refs: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

        let part_iter = stmt.query_map(&*id_refs, |row| {
            Ok(Part {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                link: row.get(2)?,
                exhibit_ids: Vec::new(), // To be populated
                notes: Vec::new(),       // To be populated
            })
        })?;

        let mut parts = Vec::new();
        for part_res in part_iter {
            let mut part = part_res?;
            let id = part.id.unwrap();

            // Fetch associated exhibit IDs
            let mut stmt_exhibits = self
                .0
                .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
            let exhibit_ids_iter = stmt_exhibits.query_map(params![id], |row| row.get(0))?;
            part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt_notes = self
                .0
                .prepare("SELECT timestamp, note FROM part_notes WHERE part_id = ?1")?;
            let notes_iter = stmt_notes.query_map(params![id], |row| {
                Ok(Note {
                    timestamp: row.get(0)?,
                    note: row.get(1)?,
                })
            })?;
            part.notes = notes_iter.collect::<Result<Vec<Note>, _>>()?;

            parts.push(part);
        }

        Ok(parts)
    }

    /// Generates and inserts 100 exhibits with associated parts and notes into the database.
    pub fn generate_and_insert_exhibits(&self) -> SqliteResult<()> {
        let clusters = vec![
            "Biology",
            "Physics",
            "Chemistry",
            "Astronomy",
            "Geology",
            "Technology",
        ];
        let locations = vec![
            "Hall A",
            "Hall B",
            "Hall C",
            "Hall D",
            "Hall E",
            "Outdoor Area",
        ];
        let statuses = vec!["operational", "needs repair", "out of service"];

        let mut rng = rand::thread_rng();

        for i in 1..=100 {
            let name = format!("Exhibit {}", i);
            let cluster = clusters[rng.gen_range(0..clusters.len())];
            let location = locations[rng.gen_range(0..locations.len())];
            let status = statuses[rng.gen_range(0..statuses.len())];
            let image_url = format!("https://picsum.photos/seed/{}/300/200", i);

            // Generate sponsor details with a 30% chance
            let (sponsor_name, sponsor_start_date, sponsor_end_date) = if rng.gen_bool(0.3) {
                let sponsor_name = Some(format!("Sponsor {}", i));
                let start_date = Some(
                    (Utc::now() - Duration::days(rng.gen_range(0..365)))
                        .format("%Y-%m-%d")
                        .to_string(),
                );
                let end_date = Some(
                    (Utc::now() + Duration::days(rng.gen_range(0..365)))
                        .format("%Y-%m-%d")
                        .to_string(),
                );
                // **Fixed Line**: Changed `sponsor_end_date` to `end_date` here
                (sponsor_name, start_date, end_date)
            } else {
                (None, None, None)
            };

            // Insert the exhibit into the database
            self.0.execute(
                "INSERT INTO exhibits (name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    name, cluster, location, status, image_url, sponsor_name, sponsor_start_date, sponsor_end_date
                ],
            )?;
            let exhibit_id = self.0.last_insert_rowid();

            // Generate and insert parts associated with the exhibit
            let num_parts = rng.gen_range(3..13);
            for j in 1..=num_parts {
                let part_name = format!("Part {} for Exhibit {}", j, i);
                let part_link = format!("https://example.com/exhibit/{}/part/{}", exhibit_id, j);
                self.0.execute(
                    "INSERT INTO parts (name, link) VALUES (?1, ?2)",
                    params![part_name, part_link],
                )?;
                let part_id = self.0.last_insert_rowid();

                // Associate part with the exhibit
                self.0.execute(
                    "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                    params![exhibit_id, part_id],
                )?;

                // Optionally, add notes to the part
                let num_part_notes = rng.gen_range(1..6);
                for k in 1..=num_part_notes {
                    let days_ago = rng.gen_range(0..365);
                    let timestamp = (Utc::now() - Duration::days(days_ago))
                        .format("%Y-%m-%d")
                        .to_string();
                    let note = format!("Note {} for Part {} of Exhibit {}", k, j, i);
                    self.0.execute(
                        "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
                        params![part_id, &timestamp, &note],
                    )?;
                }
            }

            // Generate and insert notes for the exhibit
            let num_notes = rng.gen_range(1..21);
            for j in 1..=num_notes {
                let days_ago = rng.gen_range(0..365);
                let timestamp = (Utc::now() - Duration::days(days_ago))
                    .format("%Y-%m-%d")
                    .to_string();
                let note = format!("Note {} for Exhibit {}", j, i);
                self.0.execute(
                    "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
                    params![exhibit_id, &timestamp, &note],
                )?;
            }
        }

        Ok(())
    }

    /// Wipes the database by dropping all tables.
    pub fn wipe_database(&self) -> SqliteResult<()> {
        // Drop the exhibit_parts table
        self.0.execute("DROP TABLE IF EXISTS exhibit_parts", [])?;

        // Drop the exhibit_notes table
        self.0.execute("DROP TABLE IF EXISTS exhibit_notes", [])?;

        // Drop the part_notes table
        self.0.execute("DROP TABLE IF EXISTS part_notes", [])?;

        // Drop the parts table
        self.0.execute("DROP TABLE IF EXISTS parts", [])?;

        // Drop the exhibits table
        self.0.execute("DROP TABLE IF EXISTS exhibits", [])?;

        Ok(())
    }
}
