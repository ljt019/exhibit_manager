/// Custom error type for handling various errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred with the database")]
    DatabaseError,
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("GitHub request error: {0}")]
    GitHubRequestError(String),
    #[error("GitHub API error: {0}")]
    GitHubApiError(String),
}

/// Implementing Warp's Reject trait for the custom error
impl warp::reject::Reject for Error {}
