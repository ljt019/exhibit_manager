// src/handlers/exhibit_handlers.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Reply;

use crate::db::DbConnection;
use crate::models::Exhibit;
use crate::repositories::ExhibitRepository;
use crate::Error;
use log::error;
use rand::seq::SliceRandom;

/// Handler to create a new exhibit
pub async fn create_exhibit_handler(
    new_exhibit: Exhibit,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl Reply, warp::Rejection> {
    // Initialize the ExhibitRepository
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    // Save the new exhibit to the database
    match exhibit_repo.create_exhibit(&new_exhibit) {
        Ok(id) => Ok(warp::reply::with_status(
            warp::reply::json(&id),
            StatusCode::CREATED,
        )),
        Err(e) => {
            error!("Database error: {}", e);
            Err(warp::reject::custom(Error::DatabaseError))
        }
    }
}

/// Handler to retrieve an exhibit by ID
pub async fn get_exhibit_handler(
    id: i64,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    match exhibit_repo.get_exhibit(id) {
        Ok(Some(exhibit)) => Ok(warp::reply::json(&exhibit)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to update an existing exhibit
pub async fn update_exhibit_handler(
    id: i64,
    updated_exhibit: Exhibit,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    match exhibit_repo.update_exhibit(id, &updated_exhibit) {
        Ok(updated) if updated > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::OK,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to delete an exhibit by ID
pub async fn delete_exhibit_handler(
    id: i64,
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    match exhibit_repo.delete_exhibit(id) {
        Ok(deleted) if deleted > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to list all exhibits
pub async fn list_exhibits_handler(
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    match exhibit_repo.list_exhibits() {
        Ok(exhibits) => Ok(warp::reply::json(&exhibits)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to get a random exhibit
pub async fn handle_random_exhibit(
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    match exhibit_repo.list_exhibits() {
        Ok(exhibits) => {
            if exhibits.is_empty() {
                return Err(warp::reject::not_found());
            }
            let random_exhibit = exhibits.choose(&mut rand::thread_rng()).unwrap();
            Ok(warp::reply::json(random_exhibit))
        }
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to create dummy exhibits
#[allow(dead_code)]
pub async fn create_dummy_exhibits_handler(
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;
    let exhibit_repo = ExhibitRepository::new(&*db_conn);

    match exhibit_repo.generate_and_insert_exhibits() {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!({
            "message": "Dummy exhibits created successfully"
        }))),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}
