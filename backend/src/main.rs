mod db;
mod models;

use warp::http::StatusCode;
use warp::Filter;
use warp::Reply;

use db::db_connection::DbConnection;
use db::Db;

use models::{BugReport, Exhibit, Part};

use rand::seq::SliceRandom;

use reqwest::Client;

use log::{error, info};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

use dotenv::dotenv;

use std::env;

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
    let db = std::sync::Arc::new(tokio::sync::Mutex::new(
        DbConnection::new().expect("Failed to create database connection"),
    ));

    // Generate and insert exhibits (only once or conditionally)
    {
        let test = db.lock().await;
        test.generate_and_insert_exhibits()
            .expect("Failed to generate and insert exhibits");
    }

    // Configure CORS to allow any origin and specific methods and headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    // === Image Hosting Routes ===
    let host_images = warp::path("images").and(warp::fs::dir("images"));

    // ==== Exhibit Routes ====

    // Create Exhibit: POST /exhibits
    let create_exhibit = warp::post()
        .and(warp::path("exhibits"))
        .and(warp::path::end()) // Ensure exact match
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_exhibit_handler);

    // Get Exhibit by ID: GET /exhibits/:id
    let get_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(get_exhibit_handler);

    // Update Exhibit: PUT /exhibits/:id
    let update_exhibit = warp::put()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end()) // Ensure exact match
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_exhibit_handler);

    // Delete Exhibit: DELETE /exhibits/:id
    let delete_exhibit = warp::delete()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(delete_exhibit_handler);

    // List All Exhibits: GET /exhibits
    let list_exhibits = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(list_exhibits_handler);

    let random_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path("random"))
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(handle_random_exhibit);

    // ==== Part Routes ====

    // Create Part: POST /parts
    let create_part = warp::post()
        .and(warp::path("parts"))
        .and(warp::path::end()) // Ensures exact match to /parts
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_part_handler);

    // Get Part by ID: GET /parts/:id
    let get_part = warp::get()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(get_part_handler);

    // Update Part: PUT /parts/:id
    let update_part = warp::put()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end()) // Ensure exact match
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_part_handler);

    // Delete Part: DELETE /parts/:id
    let delete_part = warp::delete()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(delete_part_handler);

    // List All Parts: GET /parts
    let list_parts = warp::get()
        .and(warp::path("parts"))
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(list_parts_handler);

    // Get Parts with list of IDs: POST /parts/batch
    let get_parts_by_ids = warp::post()
        .and(warp::path("parts"))
        .and(warp::path("batch"))
        .and(warp::path::end()) // Ensure exact match to /parts/batch
        .and(warp::body::json()) // Expecting a JSON array of i64
        .and(with_db(db.clone()))
        .and_then(get_parts_by_ids_handler);

    // Reset the database: POST /reset
    let reset_db = warp::get()
        .and(warp::path("reset"))
        .and(warp::path::end()) // Ensure exact match
        .and(with_db(db.clone()))
        .and_then(handle_reset_db);

    // Define the /report-bug route
    let report_bug = warp::post()
        .and(warp::path("report-bug"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(report_bug_handler);

    // Combine all routes
    let routes = create_exhibit
        .or(host_images)
        .or(get_exhibit)
        .or(update_exhibit)
        .or(delete_exhibit)
        .or(list_exhibits)
        .or(random_exhibit)
        .or(create_part)
        .or(get_part)
        .or(update_part)
        .or(delete_part)
        .or(list_parts)
        .or(get_parts_by_ids)
        .or(reset_db)
        .or(report_bug)
        .with(cors)
        .recover(handle_rejection);

    // Start the Warp server on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// Helper function to pass the database connection to handlers
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn handle_reset_db(db: Db) -> Result<impl Reply, warp::Rejection> {
    let db = db.lock().await;

    db.wipe_database().expect("Failed to wipe database");

    db.setup_tables().expect("Failed to setup tables");

    db.generate_and_insert_exhibits()
        .expect("Failed to generate and insert exhibits");

    Ok(warp::reply::json(&serde_json::json!({
        "message": "Database reset successful"
    })))
}

async fn handle_random_exhibit(db: Db) -> Result<impl Reply, warp::Rejection> {
    let db = db.lock().await;
    let exhibits = db.list_exhibits().expect("Failed to list exhibits");

    let random_exhibit = exhibits
        .choose(&mut rand::thread_rng())
        .expect("Failed to choose random exhibit");

    Ok(warp::reply::json(&random_exhibit))
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    if err.is_not_found() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Not Found"
        }));
        return Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND));
    }

    if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        error!("Deserialization error: {:?}", e);
        let json = warp::reply::json(&serde_json::json!({
            "error": "Invalid request body"
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    if let Some(custom_error) = err.find::<Error>() {
        error!("Internal server error: {:?}", custom_error);
        let json = warp::reply::json(&serde_json::json!({
            "error": custom_error.to_string()
        }));
        return Ok(warp::reply::with_status(
            json,
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Fallback for other errors
    error!("Unhandled rejection: {:?}", err);
    let json = warp::reply::json(&serde_json::json!({
        "error": "Internal Server Error"
    }));
    Ok(warp::reply::with_status(
        json,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

/// Save base64 image data to a file and return the filename
async fn save_image(image_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    info!("image_data: {}", image_data);

    // Strip the data URL prefix if present (e.g., "data:image/jpeg;base64,")
    let base64_data = image_data.split(",").last().unwrap_or(image_data);

    // Decode base64 data
    let image_bytes = BASE64
        .decode(base64_data)
        .expect("Failed to decode base64 data");

    // Generate a unique filename using UUID
    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
    let path = std::path::PathBuf::from("images").join(&filename);

    // Save the image file
    tokio::fs::write(&path, image_bytes).await?;

    Ok(filename)
}

/// Handler to create a new exhibit
async fn create_exhibit_handler(
    mut new_exhibit: Exhibit,
    db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Handle image upload if image_data is present
    let image_data = new_exhibit.image_url;

    match save_image(&image_data).await {
        Ok(filename) => {
            // Update image_url with the saved image path
            new_exhibit.image_url = format!("http://localhost:3030/images/{}", filename);
        }
        Err(e) => {
            error!("Failed to save image: {}", e);
            return Err(warp::reject::custom(Error::ImageProcessingError));
        }
    }

    let db = db.lock().await;

    // Save the new exhibit to the database
    match db.create_exhibit(&new_exhibit) {
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

async fn get_parts_by_ids_handler(
    part_ids: Vec<i64>,
    db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received /parts/batch request with IDs: {:?}", part_ids);

    // Check if the list is empty
    if part_ids.is_empty() {
        info!("Empty part_ids received.");
        return Ok(warp::reply::with_status(
            warp::reply::json(&Vec::<Part>::new()),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    let db = db.lock().await;
    match db.get_parts_by_ids(&part_ids) {
        Ok(parts) => {
            info!("Successfully retrieved {} parts.", parts.len());
            Ok(warp::reply::with_status(
                warp::reply::json(&parts),
                warp::http::StatusCode::OK,
            ))
        }
        Err(e) => {
            error!("Database error while fetching parts: {:?}", e);
            Err(warp::reject::custom(Error::DatabaseError))
        }
    }
}

async fn report_bug_handler(report: BugReport) -> Result<impl warp::Reply, warp::Rejection> {
    // Load GitHub credentials from environment variables
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| warp::reject::custom(Error::MissingEnvVar("GITHUB_TOKEN".to_string())))?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")
        .map_err(|_| warp::reject::custom(Error::MissingEnvVar("GITHUB_REPO_OWNER".to_string())))?;
    let repo_name = env::var("GITHUB_REPO_NAME")
        .map_err(|_| warp::reject::custom(Error::MissingEnvVar("GITHUB_REPO_NAME".to_string())))?;

    // Prepare the GitHub API URLs
    let issue_url = format!(
        "https://api.github.com/repos/{}/{}/issues",
        repo_owner, repo_name
    );
    let labels_url = format!(
        "https://api.github.com/repos/{}/{}/labels/{}",
        repo_owner,
        repo_name,
        urlencoding::encode(&report.name)
    );

    // Initialize the HTTP client
    let client = Client::new();

    // Step 1: Ensure the dynamic label exists
    let label_creation_response = client
        .get(&labels_url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "YourAppName") // Replace with your app's name
        .send()
        .await;

    match label_creation_response {
        Ok(response) => {
            if response.status().as_u16() == warp::http::StatusCode::NOT_FOUND.as_u16() {
                // Label does not exist; create it
                let create_label_url = format!(
                    "https://api.github.com/repos/{}/{}/labels",
                    repo_owner, repo_name
                );
                let label_payload = serde_json::json!({
                    "name": report.name,
                    "color": "f29513", // You can choose an appropriate color
                    "description": "Dynamic label from bug report"
                });

                let create_response = client
                    .post(&create_label_url)
                    .header("Authorization", format!("token {}", github_token))
                    .header("User-Agent", "YourAppName") // Replace with your app's name
                    .json(&label_payload)
                    .send()
                    .await;

                match create_response {
                    Ok(resp) => {
                        if !resp.status().is_success() {
                            let error_text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Unknown error".to_string());
                            error!("Failed to create label: {}", error_text);
                            return Err(warp::reject::custom(Error::GitHubApiError(error_text)));
                        }
                    }
                    Err(e) => {
                        error!("Failed to create label: {}", e);
                        return Err(warp::reject::custom(Error::GitHubRequestError(
                            e.to_string(),
                        )));
                    }
                }
            } else if !response.status().is_success() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Error checking label existence: {}", error_text);
                return Err(warp::reject::custom(Error::GitHubApiError(error_text)));
            }
            // If label exists, do nothing
        }
        Err(e) => {
            error!("Failed to check label existence: {}", e);
            return Err(warp::reject::custom(Error::GitHubRequestError(
                e.to_string(),
            )));
        }
    }

    // Step 2: Create the issue with both labels
    let payload = serde_json::json!({
        "title": format!("[bug-report] {}", report.title), // Updated title without the name
        "body": report.description,
        "labels": ["bug report", report.name]
    });

    // Send the POST request to GitHub to create the issue
    let response = client
        .post(&issue_url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "YourAppName") // Replace with your app's name
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send request to GitHub: {}", e);
            warp::reject::custom(Error::GitHubRequestError(e.to_string()))
        })?;

    // Check the response status
    if response.status().is_success() {
        let issue: serde_json::Value = response.json().await.unwrap_or_default();
        Ok(warp::reply::with_status(
            warp::reply::json(&issue),
            StatusCode::CREATED,
        ))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!("GitHub API error: {}", error_text);
        Err(warp::reject::custom(Error::GitHubApiError(error_text)))
    }
}

/// Custom error type for handling database errors
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("An error occurred with the database")]
    DatabaseError,
    #[error("Failed to process image")]
    ImageProcessingError,
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("GitHub request error: {0}")]
    GitHubRequestError(String),
    #[error("GitHub API error: {0}")]
    GitHubApiError(String),
}

/// Implementing Warp's Reject trait for the custom error
impl warp::reject::Reject for Error {}
