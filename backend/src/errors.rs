// src/errors.rs

use warp::http::StatusCode;
use warp::Reply;

/// Custom error type for handling various errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred with the database: {0}")]
    DatabaseError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("GitHub request error: {0}")]
    GitHubRequestError(String),

    #[error("GitHub API error: {0}")]
    GitHubApiError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Invalid request body")]
    InvalidRequestBody,

    #[error("Not Found")]
    NotFound,

    #[error("Unauthorized access")]
    Unauthorized,
}

/// Implementing Warp's Reject trait for the custom error
impl warp::reject::Reject for Error {}

/// Error handling for Warp rejections
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    if let Some(api_error) = err.find::<Error>() {
        let code = match api_error {
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::MissingEnvVar(_) => StatusCode::BAD_REQUEST,
            Error::GitHubRequestError(_) => StatusCode::BAD_GATEWAY,
            Error::GitHubApiError(_) => StatusCode::BAD_GATEWAY,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidRequestBody => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
        };

        let json = warp::reply::json(&serde_json::json!({
            "error": api_error.to_string(),
        }));

        return Ok(warp::reply::with_status(json, code));
    }

    if err.is_not_found() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Not Found"
        }));
        return Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND));
    }

    if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Invalid request body"
        }));
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    // Fallback for other errors
    eprintln!("Unhandled rejection: {:?}", err);
    let json = warp::reply::json(&serde_json::json!({
        "error": "Internal Server Error"
    }));
    Ok(warp::reply::with_status(
        json,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
