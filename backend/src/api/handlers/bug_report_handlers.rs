// src/handlers/bug_report_handlers.rs

use log::error;
use reqwest::Client;
use std::env;
use urlencoding::encode;
use warp::http::StatusCode;

use crate::errors::ApiError as Error;
use crate::models::BugReport;

/// Handler to report a bug via GitHub Issues
pub async fn report_bug_handler(report: BugReport) -> Result<impl warp::Reply, warp::Rejection> {
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
        encode(&report.name)
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
