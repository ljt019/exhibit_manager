use crate::db::DbConnection;
use crate::models::note::Note;
use crate::models::part::Part;
use rusqlite::OptionalExtension;
use rusqlite::{params, Result as SqliteResult};

pub struct PartRepository<'a> {
    db_conn: &'a DbConnection,
}

impl<'a> PartRepository<'a> {
    pub fn new(db_conn: &'a DbConnection) -> Self {
        PartRepository { db_conn }
    }

    pub fn create_part(&self, part: &Part) -> SqliteResult<i64> {
        self.db_conn.0.execute(
            "INSERT INTO parts (name, link) VALUES (?1, ?2)",
            params![part.name, part.link],
        )?;
        let part_id = self.db_conn.0.last_insert_rowid();

        // Associate exhibits with the part
        for exhibit_id in &part.exhibit_ids {
            self.db_conn.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![exhibit_id, part_id],
            )?;
        }

        // Insert notes related to the part
        for note in &part.notes {
            self.db_conn.0.execute(
                "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![part_id, &note.timestamp, &note.note],
            )?;
        }

        Ok(part_id)
    }

    pub fn get_part(&self, id: i64) -> SqliteResult<Option<Part>> {
        let part_opt = self
            .db_conn
            .0
            .query_row(
                "SELECT id, name, link FROM parts WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Part {
                        id: Some(row.get(0)?),
                        name: row.get(1)?,
                        link: row.get(2)?,
                        exhibit_ids: Vec::new(),
                        notes: Vec::new(),
                    })
                },
            )
            .optional()?;

        if let Some(mut part) = part_opt {
            // Fetch associated exhibit IDs
            let mut stmt = self
                .db_conn
                .0
                .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
            let exhibit_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
            part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt = self
                .db_conn
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

    pub fn update_part(&self, id: i64, part: &Part) -> SqliteResult<usize> {
        let affected = self.db_conn.0.execute(
            "UPDATE parts SET name = ?1, link = ?2 WHERE id = ?3",
            params![part.name, part.link, id],
        )?;

        // Update associated exhibits: Remove existing and add new associations
        self.db_conn
            .0
            .execute("DELETE FROM exhibit_parts WHERE part_id = ?1", params![id])?;
        for exhibit_id in &part.exhibit_ids {
            self.db_conn.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![exhibit_id, id],
            )?;
        }

        // Update notes: Remove existing and add new notes
        self.db_conn
            .0
            .execute("DELETE FROM part_notes WHERE part_id = ?1", params![id])?;
        for note in &part.notes {
            self.db_conn.0.execute(
                "INSERT INTO part_notes (part_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![id, &note.timestamp, &note.note],
            )?;
        }

        Ok(affected)
    }

    pub fn delete_part(&self, id: i64) -> SqliteResult<usize> {
        self.db_conn
            .0
            .execute("DELETE FROM parts WHERE id = ?1", params![id])
    }

    pub fn list_parts(&self) -> SqliteResult<Vec<Part>> {
        let mut stmt = self.db_conn.0.prepare("SELECT id, name, link FROM parts")?;
        let parts_iter = stmt.query_map([], |row| {
            Ok(Part {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                link: row.get(2)?,
                exhibit_ids: Vec::new(),
                notes: Vec::new(),
            })
        })?;

        let mut parts = Vec::new();
        for part_res in parts_iter {
            let mut part = part_res?;
            let id = part.id.unwrap();

            // Fetch associated exhibit IDs
            let mut stmt_exhibits = self
                .db_conn
                .0
                .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
            let exhibit_ids_iter = stmt_exhibits.query_map(params![id], |row| row.get(0))?;
            part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt_notes = self
                .db_conn
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

        // Create a string of placeholders (?, ?, ?, ...)
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            "SELECT id, name, link FROM parts WHERE id IN ({})",
            placeholders
        );

        let mut stmt = self.db_conn.0.prepare(&query)?;

        // Convert ids to a vector of references for parameter binding
        let id_refs: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

        let parts_iter = stmt.query_map(&*id_refs, |row| {
            Ok(Part {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                link: row.get(2)?,
                exhibit_ids: Vec::new(),
                notes: Vec::new(),
            })
        })?;

        let mut parts = Vec::new();
        for part_res in parts_iter {
            let mut part = part_res?;
            let id = part.id.unwrap();

            // Fetch associated exhibit IDs
            let mut stmt_exhibits = self
                .db_conn
                .0
                .prepare("SELECT exhibit_id FROM exhibit_parts WHERE part_id = ?1")?;
            let exhibit_ids_iter = stmt_exhibits.query_map(params![id], |row| row.get(0))?;
            part.exhibit_ids = exhibit_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt_notes = self
                .db_conn
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
}
