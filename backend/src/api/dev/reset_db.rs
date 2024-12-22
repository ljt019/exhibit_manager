use crate::db::DbConnection;
use crate::errors;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;

#[get("/reset")]
pub async fn handle_reset_db(
    db: &rocket::State<Mutex<DbConnection>>,
) -> Result<Json<serde_json::Value>, errors::ApiError> {
    let db_conn = db.lock().await;

    db_conn
        .wipe_database()
        .map_err(|_| errors::ApiError::DatabaseError("Database Error".to_string()))?;

    db_conn
        .setup_tables()
        .map_err(|_| errors::ApiError::DatabaseError("Database Error".to_string()))?;

    Ok(Json(serde_json::json!({
        "message": "Database reset successful"
    })))
}
