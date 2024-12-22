// src/errors.rs

use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;
use std::io::Cursor;
use thiserror::Error;

#[allow(dead_code)]
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

/// Structure for API error responses
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Implement Responder for ApiError to convert it into HTTP responses
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> Result<Response<'static>, Status> {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        let status = match self {
            ApiError::DatabaseError(_) => Status::InternalServerError,
            ApiError::MissingEnvVar(_) => Status::BadRequest,
            ApiError::GitHubRequestError(_) => Status::BadGateway,
            ApiError::GitHubApiError(_) => Status::BadGateway,
            ApiError::InternalServerError => Status::InternalServerError,
            ApiError::InvalidRequestBody => Status::BadRequest,
            ApiError::NotFound => Status::NotFound,
            ApiError::Unauthorized => Status::Unauthorized,
        };

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(
                error_response.error.len(),
                Cursor::new(rocket::serde::json::serde_json::to_string(&error_response).unwrap()),
            )
            .ok()
    }
}

/// Error catchers

#[rocket::catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Not Found".into(),
    })
}

#[rocket::catch(400)]
pub fn handle_invalid_request_body() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Invalid request body".into(),
    })
}

#[rocket::catch(405)]
pub fn handle_method_not_allowed() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Method Not Allowed".into(),
    })
}

#[rocket::catch(500)]
pub fn internal_server_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Internal Server Error".into(),
    })
}
