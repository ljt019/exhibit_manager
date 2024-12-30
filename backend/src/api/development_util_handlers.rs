use crate::db;
use crate::db::DbPool;
use crate::errors::ApiError;
use log::error;
use rocket::get;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::query;

/// Generates and inserts 100 dummy exhibits into the database.
///
/// This function creates 100 dummy exhibits with predefined data and inserts them into the `exhibits` table.
/// Each exhibit is associated with parts and notes.
///
/// # Arguments
/// * `pool` - A reference to the database connection pool.
///
/// # Returns
/// * `Result<(), ApiError>` - Returns `Ok(())` if all exhibits are inserted successfully.
/// * `ApiError` - Returns an error if any database operation fails.
///
/// WARNING FOR FUTURE Self
/// MIGHT CAN FAIL IF RAND NUMBERS ARE NOT UNIQUE UNLIKELY BUT POSSIBLE
pub async fn generate_and_insert_exhibits(pool: &DbPool) -> Result<(), ApiError> {
    for _i in 1..=100 {
        let exhibit = crate::dev::get_random_dummy_exhibit();
        crate::repo::exhibit_repo::create_exhibit(&exhibit, pool).await?;
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
/// - A database operation fails.
#[get("/exhibits/fill-dummy")]
pub async fn create_dummy_exhibits_handler(
    db_pool: &State<DbPool>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let pool = db_pool.inner().clone();

    generate_and_insert_exhibits(&pool).await.map_err(|e| {
        error!("Failed to insert dummy exhibits: {}", e);
        ApiError::DatabaseError("Failed to insert dummy exhibits".into())
    })?;

    Ok(Json(serde_json::json!({
        "message": "Dummy exhibits created successfully"
    })))
}

/// Resets the database by wiping all data and setting up tables.
///
/// This function performs a complete reset of the database by removing all existing data
/// and reinitializing the necessary tables.
///
/// # Arguments
/// * `pool` - A reference to the database connection pool.
///
/// # Returns
/// * `Result<(), ApiError>` - Returns `Ok(())` if the reset is successful.
/// * `ApiError` - Returns an error if any database operation fails.
pub async fn reset_database(pool: &DbPool) -> Result<(), ApiError> {
    wipe_database(pool).await?;
    db::setup_database(pool).await.map_err(|e| {
        error!("Failed to setup database: {}", e);
        ApiError::DatabaseError("Failed to setup database".into())
    })?;

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
/// - A database operation fails.
#[get("/reset")]
pub async fn handle_reset_db(db_pool: &State<DbPool>) -> Result<Json<serde_json::Value>, ApiError> {
    let pool = db_pool.inner().clone();

    reset_database(&pool).await.map_err(|e| {
        error!("Failed to reset database: {}", e);
        ApiError::DatabaseError("Failed to reset database".into())
    })?;

    Ok(Json(serde_json::json!({
        "message": "Database reset successful"
    })))
}

/// Wipes the database by dropping all tables.
async fn wipe_database(pool: &DbPool) -> Result<(), ApiError> {
    query("DROP TABLE IF EXISTS exhibit_parts")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to drop exhibit_parts table: {}", e);
            ApiError::DatabaseError("Failed to drop exhibit_parts table".into())
        })?;

    query("DROP TABLE IF EXISTS exhibit_notes")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to drop exhibit_notes table: {}", e);
            ApiError::DatabaseError("Failed to drop exhibit_notes table".into())
        })?;

    query("DROP TABLE IF EXISTS part_notes")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to drop part_notes table: {}", e);
            ApiError::DatabaseError("Failed to drop part_notes table".into())
        })?;

    query("DROP TABLE IF EXISTS parts")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to drop parts table: {}", e);
            ApiError::DatabaseError("Failed to drop parts table".into())
        })?;

    query("DROP TABLE IF EXISTS exhibits")
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to drop exhibits table: {}", e);
            ApiError::DatabaseError("Failed to drop exhibits table".into())
        })?;

    Ok(())
}
