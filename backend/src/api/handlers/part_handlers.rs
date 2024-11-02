// src/handlers/part_handlers.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

use crate::db::repositories::PartRepository;
use crate::db::DbConnection;
use crate::errors::ApiError;
use crate::models::Part;
use log::{error, info};

/// Handler to create a new part
pub async fn create_part_handler(
    new_part: Part,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let part_repo = PartRepository::new(&*db_conn);

    match part_repo.create_part(&new_part) {
        Ok(id) => Ok(warp::reply::json(&id)),
        Err(e) => {
            error!("Database error: {}", e);
            Err(warp::reject::custom(ApiError::DatabaseError(
                "Database Error".to_string(),
            )))
        }
    }
}

/// Handler to retrieve a part by ID
pub async fn get_part_handler(
    id: i64,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let part_repo = PartRepository::new(&*db);

    match part_repo.get_part(id) {
        Ok(Some(part)) => Ok(warp::reply::with_status(
            warp::reply::json(&part),
            StatusCode::OK,
        )),
        Ok(None) => Err(warp::reject::custom(ApiError::NotFound)),
        Err(_) => Err(warp::reject::custom(ApiError::DatabaseError(
            "Database Error".to_string(),
        ))),
    }
}

/// Handler to update an existing part
pub async fn update_part_handler(
    id: i64,
    updated_part: Part,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let part_repo = PartRepository::new(&*db_conn);

    match part_repo.update_part(id, &updated_part) {
        Ok(updated) if updated > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::OK,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(ApiError::DatabaseError(
            "Database Error".to_string(),
        ))),
    }
}

/// Handler to delete a part by ID
pub async fn delete_part_handler(
    id: i64,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let part_repo = PartRepository::new(&*db_conn);

    match part_repo.delete_part(id) {
        Ok(deleted) if deleted > 0 => Ok(warp::reply::with_status(
            warp::reply(),
            StatusCode::NO_CONTENT,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(ApiError::DatabaseError(
            "Database Error".to_string(),
        ))),
    }
}

/// Handler to list all parts
pub async fn list_parts_handler(
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let part_repo = PartRepository::new(&*db_conn);

    match part_repo.list_parts() {
        Ok(parts) => Ok(warp::reply::json(&parts)),
        Err(_) => Err(warp::reject::custom(ApiError::DatabaseError(
            "Database Error".to_string(),
        ))),
    }
}

/// Handler to get parts by a list of IDs
pub async fn get_parts_by_ids_handler(
    part_ids: Vec<i64>,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received /parts/batch request with IDs: {:?}", part_ids);

    if part_ids.is_empty() {
        info!("Empty part_ids received.");
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "error": "No part IDs provided"
            })),
            StatusCode::BAD_REQUEST,
        ));
    }

    let db_conn = db.lock().await;
    let part_repo = PartRepository::new(&*db_conn);

    match part_repo.get_parts_by_ids(&part_ids) {
        Ok(parts) => {
            info!("Successfully retrieved {} parts.", parts.len());
            Ok(warp::reply::with_status(
                warp::reply::json(&parts),
                StatusCode::OK,
            ))
        }
        Err(e) => {
            error!("Database error while fetching parts: {:?}", e);
            Err(warp::reject::custom(ApiError::DatabaseError(
                "Database Error".to_string(),
            )))
        }
    }
}
