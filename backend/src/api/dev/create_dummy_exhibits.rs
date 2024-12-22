use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::models::Note;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rusqlite::Result as SqliteResult;

/// Generate and insert 100 dummy exhibits
pub fn generate_and_insert_exhibits(db_conn: &DbConnection) -> SqliteResult<()> {
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

        crate::api::exhibits::create_exhibit::create_exhibit(&exhibit, db_conn)?;
    }
    Ok(())
}

/// Handles the POST /exhibits/dummy endpoint
#[post("/exhibits/dummy")]
pub async fn create_dummy_exhibits_handler(
    db: &State<Mutex<DbConnection>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let db_conn = db.lock().await;

    match generate_and_insert_exhibits(&*db_conn) {
        Ok(_) => Ok(Json(serde_json::json!({
            "message": "Dummy exhibits created successfully"
        }))),
        Err(_) => Err(ApiError::DatabaseError("Database Error".to_string())),
    }
}
