// src/main.rs

mod api;
mod db;
mod errors;
mod models;

use errors::Error;

use warp::Filter;
use warp::Reply;

use db::DbConnection;

use log::error;

use dotenv::dotenv;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Check if images directory exists, if not create it
    if !std::path::Path::new("images").exists() {
        std::fs::create_dir("images").expect("Failed to create images directory");
    }

    // Initialize the database connection wrapped in Arc and Mutex for thread-safe access
    let db = Arc::new(Mutex::new(
        DbConnection::new("exhibits.db").expect("Failed to create database connection"),
    ));

    // Set up the database tables
    {
        let db_conn = db.lock().await;
        db_conn.setup_tables().expect("Failed to set up tables");
    }

    // Configure CORS to allow any origin and specific methods and headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    // Import route modules
    let exhibit_routes = api::routes::exhibit_routes(db.clone());
    let part_routes = api::routes::part_routes(db.clone());
    let bug_report_routes = api::routes::bug_report_routes();

    // Additional routes that don't fit into resource-specific categories
    let host_images = warp::path("images").and(warp::fs::dir("images"));
    let reset_db = warp::get()
        .and(warp::path("reset"))
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(handle_reset_db);

    // Combine all routes
    let routes = exhibit_routes
        .or(part_routes)
        .or(bug_report_routes)
        .or(host_images)
        .or(reset_db)
        .with(cors)
        .recover(handle_rejection);

    // Start the Warp server on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// Helper function to pass the database connection to handlers
fn with_db(
    db: Arc<Mutex<DbConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<DbConnection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// Error handling for Warp rejections
async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    if err.is_not_found() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Not Found"
        }));
        return Ok(warp::reply::with_status(
            json,
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        error!("Deserialization error: {:?}", e);
        let json = warp::reply::json(&serde_json::json!({
            "error": "Invalid request body"
        }));
        return Ok(warp::reply::with_status(
            json,
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    if let Some(custom_error) = err.find::<Error>() {
        error!("Internal server error: {:?}", custom_error);
        let json = warp::reply::json(&serde_json::json!({
            "error": custom_error.to_string()
        }));
        return Ok(warp::reply::with_status(
            json,
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Fallback for other errors
    error!("Unhandled rejection: {:?}", err);
    let json = warp::reply::json(&serde_json::json!({
        "error": "Internal Server Error"
    }));
    Ok(warp::reply::with_status(
        json,
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

/// Handler to reset the database
async fn handle_reset_db(
    db: Arc<Mutex<DbConnection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let db_conn = db.lock().await;

    db_conn
        .wipe_database()
        .map_err(|_| warp::reject::custom(Error::DatabaseError("Database Error".to_string())))?;

    db_conn
        .setup_tables()
        .map_err(|_| warp::reject::custom(Error::DatabaseError("Database Error".to_string())))?;

    Ok(warp::reply::json(&serde_json::json!({
        "message": "Database reset successful"
    })))
}
