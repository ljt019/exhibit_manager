mod db;
mod models;

use warp::Filter;

use db::db_connection::DbConnection;
use db::Db;

use models::{Exhibit, Part};

#[tokio::main]
async fn main() {
    // Initialize the database connection wrapped in Arc and Mutex for thread-safe access
    let db = std::sync::Arc::new(tokio::sync::Mutex::new(
        DbConnection::new().expect("Failed to create database connection"),
    ));

    // Generate and insert exhibits (only once or conditionally)
    let test = db.lock().await;
    test.generate_and_insert_exhibits()
        .expect("Failed to generate and insert exhibits");
    drop(test);

    // Configure CORS to allow any origin and specific methods and headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type"]);

    // ==== Exhibit Routes ====

    // Create Exhibit: POST /exhibits
    let create_exhibit = warp::post()
        .and(warp::path("exhibits"))
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_exhibit_handler);

    // Get Exhibit by ID: GET /exhibits/:id
    let get_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(with_db(db.clone()))
        .and_then(get_exhibit_handler);

    // Update Exhibit: PUT /exhibits/:id
    let update_exhibit = warp::put()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_exhibit_handler);

    // Delete Exhibit: DELETE /exhibits/:id
    let delete_exhibit = warp::delete()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(with_db(db.clone()))
        .and_then(delete_exhibit_handler);

    // List All Exhibits: GET /exhibits
    let list_exhibits = warp::get()
        .and(warp::path("exhibits"))
        .and(with_db(db.clone()))
        .and_then(list_exhibits_handler);

    // ==== Part Routes ====

    // Create Part: POST /parts
    let create_part = warp::post()
        .and(warp::path("parts"))
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_part_handler);

    // Get Part by ID: GET /parts/:id
    let get_part = warp::get()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(with_db(db.clone()))
        .and_then(get_part_handler);

    // Update Part: PUT /parts/:id
    let update_part = warp::put()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_part_handler);

    // Delete Part: DELETE /parts/:id
    let delete_part = warp::delete()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(with_db(db.clone()))
        .and_then(delete_part_handler);

    // List All Parts: GET /parts
    let list_parts = warp::get()
        .and(warp::path("parts"))
        .and(with_db(db.clone()))
        .and_then(list_parts_handler);

    // Combine all routes
    let routes = create_exhibit
        .or(get_exhibit)
        .or(update_exhibit)
        .or(delete_exhibit)
        .or(list_exhibits)
        .or(create_part)
        .or(get_part)
        .or(update_part)
        .or(delete_part)
        .or(list_parts)
        .with(cors);

    // Start the Warp server on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// Helper function to pass the database connection to handlers
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// Handler to create a new exhibit
async fn create_exhibit_handler(
    new_exhibit: Exhibit,
    db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.create_exhibit(&new_exhibit) {
        Ok(id) => Ok(warp::reply::json(&id)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to retrieve an exhibit by ID
async fn get_exhibit_handler(id: i64, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.get_exhibit(id) {
        Ok(Some(exhibit)) => Ok(warp::reply::json(&exhibit)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to update an existing exhibit
async fn update_exhibit_handler(
    id: i64,
    updated_exhibit: Exhibit,
    db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.update_exhibit(id, &updated_exhibit) {
        Ok(updated) if updated > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::OK,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to delete an exhibit by ID
async fn delete_exhibit_handler(id: i64, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.delete_exhibit(id) {
        Ok(deleted) if deleted > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::NO_CONTENT,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to list all exhibits
async fn list_exhibits_handler(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.list_exhibits() {
        Ok(exhibits) => Ok(warp::reply::json(&exhibits)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to create a new part
async fn create_part_handler(new_part: Part, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.create_part(&new_part) {
        Ok(id) => Ok(warp::reply::json(&id)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to retrieve a part by ID
async fn get_part_handler(id: i64, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.get_part(id) {
        Ok(Some(part)) => Ok(warp::reply::json(&part)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to update an existing part
async fn update_part_handler(
    id: i64,
    updated_part: Part,
    db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.update_part(id, &updated_part) {
        Ok(updated) if updated > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::OK,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to delete a part by ID
async fn delete_part_handler(id: i64, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.delete_part(id) {
        Ok(deleted) if deleted > 0 => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::NO_CONTENT,
        )),
        Ok(_) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Handler to list all parts
async fn list_parts_handler(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match db.list_parts() {
        Ok(parts) => Ok(warp::reply::json(&parts)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseError)),
    }
}

/// Custom error type for handling database errors
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("An error occurred with the database")]
    DatabaseError,
}

/// Implementing Warp's Reject trait for the custom error
impl warp::reject::Reject for Error {}
