// src/api/tests/routes_tests/bug_report_routes_tests.rs

use crate::api::routes::bug_report_routes;
use crate::db::DbConnection;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;

#[tokio::test]
async fn test_report_bug_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Initialize the routes
    let api = bug_report_routes().recover(crate::errors::handle_rejection);

    // Define the bug report payload
    let bug_report = json!({
        "name": "Test Bug",
        "title": "Bug in exhibit creation",
        "description": "There is a bug when creating an exhibit."
    });

    // Perform the request
    let resp = warp::test::request()
        .method("POST")
        .path("/report-bug")
        .json(&bug_report)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Optionally, assert the response body
    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body.get("url").is_some()); // Assuming GitHub returns the issue URL
}

#[tokio::test]
async fn test_report_bug_invalid_payload() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Initialize the routes
    let api = bug_report_routes().recover(crate::errors::handle_rejection);

    // Define an invalid bug report payload (missing 'description')
    let invalid_bug_report = json!({
        "name": "Test Bug",
        "title": "Bug in exhibit creation"
        // "description" is missing
    });

    // Perform the request
    let resp = warp::test::request()
        .method("POST")
        .path("/report-bug")
        .json(&invalid_bug_report)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Optionally, assert the response body
    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(resp_body["error"], "Invalid request body");
}
