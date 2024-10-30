use crate::db::DbConnection;
use crate::models::note::Note;
use rusqlite::{params, Result as SqliteResult};

pub struct NoteRepository<'a> {
    db_conn: &'a DbConnection,
}

impl<'a> NoteRepository<'a> {
    pub fn new(db_conn: &'a DbConnection) -> Self {
        NoteRepository { db_conn }
    }

    // Implement CRUD operations for notes if needed
    // For example:
    pub fn create_exhibit_note(&self, exhibit_id: i64, note: &Note) -> SqliteResult<i64> {
        self.db_conn.0.execute(
            "INSERT INTO exhibit_notes (exhibit_id, timestamp, note) VALUES (?1, ?2, ?3)",
            params![exhibit_id, &note.timestamp, &note.note],
        )?;
        Ok(self.db_conn.0.last_insert_rowid())
    }

    pub fn get_exhibit_notes(&self, exhibit_id: i64) -> SqliteResult<Vec<Note>> {
        let mut stmt = self
            .db_conn
            .0
            .prepare("SELECT timestamp, note FROM exhibit_notes WHERE exhibit_id = ?1")?;
        let notes_iter = stmt.query_map(params![exhibit_id], |row| {
            Ok(Note {
                timestamp: row.get(0)?,
                note: row.get(1)?,
            })
        })?;

        let mut notes = Vec::new();
        for note_res in notes_iter {
            notes.push(note_res?);
        }

        Ok(notes)
    }

    // Similarly, implement methods for part_notes if needed
}
