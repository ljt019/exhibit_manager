// src/api/tests/routes_tests/parts_routes_tests.rs

use crate::api::routes::part_routes;
use crate::db::DbConnection;
use crate::models::Part;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;

/// Helper function to create a test `Part`
fn get_test_part() -> Part {
    Part {
        id: None,
        name: "Test Part".to_string(),
        link: "https://www.example.com/test-part".to_string(),
        exhibit_ids: vec![],
        notes: vec![],
    }
}

#[tokio::test]
async fn test_create_part_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Define the part payload using the test helper
    let part = json!({
        "name": get_test_part().name,
        "link": get_test_part().link,
        "exhibit_ids": get_test_part().exhibit_ids,
        "notes": get_test_part().notes,
    });

    // Perform the request
    let resp = warp::test::request()
        .method("POST")
        .path("/parts")
        .json(&part)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert the response body contains the new part ID
    let resp_body: i64 = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body > 0);
}

#[tokio::test]
async fn test_get_part_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert a sample part
    let db_conn = db.lock().await;
    let part_repo = crate::db::repositories::PartRepository::new(&*db_conn);
    let part_id = part_repo.create_part(&get_test_part()).unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the request
    let resp = warp::test::request()
        .method("GET")
        .path(&format!("/parts/{}", part_id))
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert the response body contains the correct part data
    let resp_body: Part = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(resp_body.id.unwrap(), part_id);
    assert_eq!(resp_body.name, "Test Part");
    assert_eq!(resp_body.link, "https://www.example.com/test-part");
}

#[tokio::test]
async fn test_update_part_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert a sample part
    let db_conn = db.lock().await;
    let part_repo = crate::db::repositories::PartRepository::new(&*db_conn);
    let part_id = part_repo.create_part(&get_test_part()).unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Define the updated part payload
    let updated_part = json!({
        "name": "Updated Test Part",
        "link": "https://www.example.com/updated-test-part",
        "exhibit_ids": [],
        "notes": [
            { "timestamp": "2024-01-01T00:00:00Z", "note": "First note" },
            { "timestamp": "2024-01-02T00:00:00Z", "note": "Second note" }
        ],
    });

    // Perform the update request
    let resp = warp::test::request()
        .method("PUT")
        .path(&format!("/parts/{}", part_id))
        .json(&updated_part)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Retrieve the part to verify updates
    let get_resp = warp::test::request()
        .method("GET")
        .path(&format!("/parts/{}", part_id))
        .reply(&api)
        .await;

    let retrieved_part: Part = serde_json::from_slice(get_resp.body()).unwrap();
    assert_eq!(retrieved_part.name, "Updated Test Part");
    assert_eq!(
        retrieved_part.link,
        "https://www.example.com/updated-test-part"
    );
    assert_eq!(retrieved_part.notes.len(), 2);
    assert_eq!(retrieved_part.notes[0].note, "First note");
    assert_eq!(retrieved_part.notes[1].note, "Second note");
}

#[tokio::test]
async fn test_delete_part_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert a sample part
    let db_conn = db.lock().await;
    let part_repo = crate::db::repositories::PartRepository::new(&*db_conn);
    let part_id = part_repo.create_part(&get_test_part()).unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the delete request
    let resp = warp::test::request()
        .method("DELETE")
        .path(&format!("/parts/{}", part_id))
        .reply(&api)
        .await;

    println!("{:?}", resp.status());

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Attempt to retrieve the deleted part
    let get_resp = warp::test::request()
        .method("GET")
        .path(&format!("/parts/{}", part_id))
        .reply(&api)
        .await;

    println!("{:?}", get_resp.status());

    // Assert that the part is not found
    assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_parts_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert multiple parts
    let db_conn = db.lock().await;
    let part_repo = crate::db::repositories::PartRepository::new(&*db_conn);

    let part1 = get_test_part();
    let part2 = Part {
        id: None,
        name: "Second Test Part".to_string(),
        link: "https://www.example.com/second-test-part".to_string(),
        exhibit_ids: vec![],
        notes: vec![crate::models::Note {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            note: "Initial note".to_string(),
        }],
    };

    let _id1 = part_repo.create_part(&part1).unwrap();
    let _id2 = part_repo.create_part(&part2).unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the list request
    let resp = warp::test::request()
        .method("GET")
        .path("/parts")
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert that the response body contains a list of parts
    let parts: Vec<Part> = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].name, "Test Part");
    assert_eq!(parts[1].name, "Second Test Part");
}

#[tokio::test]
async fn test_get_parts_by_ids_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert multiple parts and collect their IDs
    let db_conn = db.lock().await;
    let part_repo = crate::db::repositories::PartRepository::new(&*db_conn);

    let part1 = get_test_part();
    let part2 = Part {
        id: None,
        name: "Second Test Part".to_string(),
        link: "https://www.example.com/second-test-part".to_string(),
        exhibit_ids: vec![],
        notes: vec![crate::models::Note {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            note: "Initial note".to_string(),
        }],
    };

    let id1 = part_repo.create_part(&part1).unwrap();
    let id2 = part_repo.create_part(&part2).unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Define the list of part IDs to retrieve
    let part_ids = json!([id1, id2]);

    // Perform the batch get request
    let resp = warp::test::request()
        .method("POST")
        .path("/parts/batch")
        .json(&part_ids)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert that the response body contains the requested parts
    let retrieved_parts: Vec<Part> = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(retrieved_parts.len(), 2);
    assert_eq!(retrieved_parts[0].id.unwrap(), id1);
    assert_eq!(retrieved_parts[0].name, "Test Part");
    assert_eq!(retrieved_parts[1].id.unwrap(), id2);
    assert_eq!(retrieved_parts[1].name, "Second Test Part");
}

#[tokio::test]
async fn test_get_parts_by_ids_empty_list() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Initialize the routes
    let api = part_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Define an empty list of part IDs
    let part_ids = json!([]);

    // Perform the batch get request
    let resp = warp::test::request()
        .method("POST")
        .path("/parts/batch")
        .json(&part_ids)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Assert the response body contains an error message
    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(resp_body["error"], "No part IDs provided");
}
