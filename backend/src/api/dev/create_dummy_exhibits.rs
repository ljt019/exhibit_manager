use crate::db::DbPool;
use crate::errors::ApiError;
use crate::models::Exhibit;
use crate::models::Note;
use log::error;
use rocket::get;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;

/// Generates and inserts 100 dummy exhibits into the database.
///
/// This function creates 100 dummy exhibits with predefined data and inserts them into the `exhibits` table.
/// Each exhibit is associated with parts and notes.
///
/// # Arguments
/// * `conn` - A reference to the database connection.
///
/// # Returns
/// * `rusqlite::Result<()>` - Returns `Ok(())` if all exhibits are inserted successfully.
/// * `rusqlite::Error` - Returns an error if any database operation fails.
pub fn generate_and_insert_exhibits(conn: &Connection) -> rusqlite::Result<()> {
    for i in 1..=100 {
        let exhibit = Exhibit {
            id: None,
            name: format!("Exhibit {}", i),
            cluster: format!("Cluster {}", (i % 10) + 1),
            location: format!("Location {}", (i % 5) + 1),
            status: "Operational".to_string(),
            image_url: format!("https://picsum.photos/seed/picsum/200/300"),
            sponsor_name: Some(format!("Sponsor {}", (i % 3) + 1)),
            sponsor_start_date: Some("2023-01-01".to_string()),
            sponsor_end_date: Some("2023-12-31".to_string()),
            part_ids: vec![],
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

        crate::api::exhibits::create_exhibit::create_exhibit(&exhibit, conn)?;
    }
    Ok(())
}

/// Handles the GET /exhibits/dummy endpoint.
///
/// This endpoint is only a GET and not a POST because it's easier to test with the frontend.
///
/// This endpoint generates and inserts 100 dummy exhibits into the database.
/// It returns a success message upon successful creation.
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
#[get("/exhibits/fill-dummy")]
pub async fn create_dummy_exhibits_handler(
    db_pool: &State<DbPool>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let pool = (*db_pool).clone();

    // Offload the blocking database operation to a separate thread
    let _ = rocket::tokio::task::spawn_blocking(move || {
        let conn = pool.get().map_err(|_| {
            error!("Failed to get DB connection from pool");
            ApiError::DatabaseError("Failed to get DB connection".into())
        })?;
        generate_and_insert_exhibits(&conn).map_err(|e| {
            error!("Failed to insert dummy exhibits: {}", e);
            ApiError::DatabaseError("Failed to insert dummy exhibits".into())
        })
    })
    .await
    .map_err(|e| {
        error!("Task panicked while inserting dummy exhibits: {}", e);
        ApiError::DatabaseError("Internal Server Error".into())
    })??;

    Ok(Json(serde_json::json!({
        "message": "Dummy exhibits created successfully"
    })))
}
