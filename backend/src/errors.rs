// src/errors.rs

use serde::Serialize;
use thiserror::Error;
use warp::reject::MethodNotAllowed;
use warp::{http::StatusCode, Rejection, Reply};

/// Unified API error type
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("GitHub request error: {0}")]
    GitHubRequestError(String),

    #[error("GitHub API error: {0}")]
    GitHubApiError(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Invalid request body")]
    InvalidRequestBody,

    #[error("Not Found")]
    NotFound,

    #[error("Unauthorized access")]
    Unauthorized,
}

impl warp::reject::Reject for ApiError {}

/// Structure for API error responses
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Converts ApiError to a standardized ErrorResponse
impl ApiError {
    fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.to_string(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MissingEnvVar(_) => StatusCode::BAD_REQUEST,
            ApiError::GitHubRequestError(_) => StatusCode::BAD_GATEWAY,
            ApiError::GitHubApiError(_) => StatusCode::BAD_GATEWAY,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidRequestBody => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

/// Handles all rejections and converts them into standardized API responses
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(api_error) = err.find::<ApiError>() {
        let response = api_error.to_response();
        let status = api_error.status_code();
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        ));
    }

    if err.is_not_found() {
        let response = ErrorResponse {
            error: "Not Found".into(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::NOT_FOUND,
        ));
    }

    if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        let response = ErrorResponse {
            error: "Invalid request body".into(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::BAD_REQUEST,
        ));
    }

    if let Some(_) = err.find::<MethodNotAllowed>() {
        let response = ErrorResponse {
            error: "Method Not Allowed".into(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::METHOD_NOT_ALLOWED,
        ));
    }

    // Fallback for other errors
    eprintln!("Unhandled rejection: {:?}", err);
    let response = ErrorResponse {
        error: "Internal Server Error".into(),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
