// src/repositories/exhibit_repository.rs

use crate::db::DbConnection;
use crate::models::{Exhibit, Note};
use rusqlite::OptionalExtension;
use rusqlite::{params, Result as SqliteResult};

pub struct ExhibitRepository<'a> {
    db_conn: &'a DbConnection,
}

impl<'a> ExhibitRepository<'a> {
    pub fn new(db_conn: &'a DbConnection) -> Self {
        ExhibitRepository { db_conn }
    }

    pub fn create_exhibit(&self, exhibit: &Exhibit) -> SqliteResult<i64> {
        self.db_conn.0.execute(
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
        let exhibit_id = self.db_conn.0.last_insert_rowid();

        // Associate parts with the exhibit
        for part_id in &exhibit.part_ids {
            self.db_conn.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![exhibit_id, part_id],
            )?;
        }

        // Insert notes related to the exhibit
        for note in &exhibit.notes {
            self.db_conn.0.execute(
                "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![exhibit_id, &note.timestamp, &note.note],
            )?;
        }

        Ok(exhibit_id)
    }

    pub fn get_exhibit(&self, id: i64) -> SqliteResult<Option<Exhibit>> {
        let exhibit_opt = self.db_conn.0
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
                .db_conn
                .0
                .prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
            let part_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
            exhibit.part_ids = part_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt = self
                .db_conn
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

    pub fn update_exhibit(&self, id: i64, exhibit: &Exhibit) -> SqliteResult<usize> {
        let affected = self.db_conn.0.execute(
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
        self.db_conn.0.execute(
            "DELETE FROM exhibit_parts WHERE exhibit_id = ?1",
            params![id],
        )?;
        for part_id in &exhibit.part_ids {
            self.db_conn.0.execute(
                "INSERT INTO exhibit_parts (exhibit_id, part_id) VALUES (?1, ?2)",
                params![id, part_id],
            )?;
        }

        // Update notes: Remove existing and add new notes
        self.db_conn.0.execute(
            "DELETE FROM exhibit_notes WHERE exhibit_id = ?1",
            params![id],
        )?;
        for note in &exhibit.notes {
            self.db_conn.0.execute(
                "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
                params![id, &note.timestamp, &note.note],
            )?;
        }

        Ok(affected)
    }

    pub fn delete_exhibit(&self, id: i64) -> SqliteResult<usize> {
        self.db_conn
            .0
            .execute("DELETE FROM exhibits WHERE id = ?1", params![id])
    }

    pub fn list_exhibits(&self) -> SqliteResult<Vec<Exhibit>> {
        let mut stmt = self.db_conn.0.prepare(
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
                part_ids: Vec::new(),
                notes: Vec::new(),
            })
        })?;

        let mut exhibits = Vec::new();
        for exhibit_res in exhibits_iter {
            let mut exhibit = exhibit_res?;
            let id = exhibit.id.unwrap();

            // Fetch associated part IDs
            let mut stmt_parts = self
                .db_conn
                .0
                .prepare("SELECT part_id FROM exhibit_parts WHERE exhibit_id = ?1")?;
            let part_ids_iter = stmt_parts.query_map(params![id], |row| row.get(0))?;
            exhibit.part_ids = part_ids_iter.collect::<Result<Vec<i64>, _>>()?;

            // Fetch associated notes
            let mut stmt_notes = self
                .db_conn
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

    /// **New Method**: Generate and insert 100 dummy exhibits
    pub fn generate_and_insert_exhibits(&self) -> SqliteResult<()> {
        for i in 1..=100 {
            let exhibit = Exhibit {
                id: None,
                name: format!("Exhibit {}", i),
                cluster: format!("Cluster {}", (i % 10) + 1),
                location: format!("Location {}", (i % 5) + 1),
                status: "active".to_string(),
                image_url: format!("http://localhost:3030/images/{}.jpg", i),
                sponsor_name: Some(format!("Sponsor {}", (i % 3) + 1)),
                sponsor_start_date: Some("2023-01-01".to_string()),
                sponsor_end_date: Some("2023-12-31".to_string()),
                part_ids: vec![i as i64, (i + 1) as i64],
                notes: vec![
                    Note {
                        timestamp: "2023-10-01".to_string(),
                        note: format!("Note 1 for Exhibit {}", i),
                    },
                    Note {
                        timestamp: "2023-10-02".to_string(),
                        note: format!("Note 2 for Exhibit {}", i),
                    },
                ],
            };

            self.create_exhibit(&exhibit)?;
        }
        Ok(())
    }
}
