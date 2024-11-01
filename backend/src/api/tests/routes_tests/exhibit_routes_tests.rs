// src/api/tests/routes_tests/exhibit_routes_tests.rs

use crate::api::routes::exhibit_routes;
use crate::db::DbConnection;
use crate::models::Exhibit;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;

fn get_test_exhibit() -> Exhibit {
    let exhibit = Exhibit {
        id: None,
        name: "Test Exhibit".to_string(),
        cluster: "Test Cluster".to_string(),
        location: "Test Location".to_string(),
        status: "Test Status".to_string(),
        image_url: "Test Image URL".to_string(),
        sponsor_name: None,
        sponsor_start_date: None,
        sponsor_end_date: None,
        part_ids: vec![],
        notes: vec![],
    };

    exhibit
}

#[tokio::test]
async fn test_create_exhibit_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Initialize the routes
    let api = exhibit_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Define the exhibit payload
    let exhibit = json!({
        "name": "Art Exhibit",
        "cluster": "Modern Art",
        "location": "Gallery 1",
        "status": "Active",
        "image_url": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD...", // Sample base64
        "sponsor_name": "Art Sponsor",
        "sponsor_start_date": "2024-01-01",
        "sponsor_end_date": "2024-12-31",
        "part_ids": [],
        "notes": []
    });

    // Perform the request
    let resp = warp::test::request()
        .method("POST")
        .path("/exhibits")
        .json(&exhibit)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Optionally, assert the response body contains the new exhibit ID
    let resp_body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
    assert!(resp_body.as_i64().is_some());
}

#[tokio::test]
async fn test_get_exhibit_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert a sample exhibit
    let db_conn = db.lock().await;
    let exhibit_repo = crate::db::repositories::ExhibitRepository::new(&*db_conn);
    let exhibit_id = exhibit_repo
        .create_exhibit(&crate::models::Exhibit {
            id: None,
            name: "Art Exhibit".to_string(),
            cluster: "Modern Art".to_string(),
            location: "Gallery 1".to_string(),
            status: "Active".to_string(),
            image_url: "http://localhost:3030/images/sample.jpg".to_string(),
            sponsor_name: Some("Art Sponsor".to_string()),
            sponsor_start_date: Some("2024-01-01".to_string()),
            sponsor_end_date: Some("2024-12-31".to_string()),
            part_ids: vec![],
            notes: vec![],
        })
        .unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = exhibit_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the request
    let resp = warp::test::request()
        .method("GET")
        .path(&format!("/exhibits/{}", exhibit_id))
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert the response body contains the correct exhibit data
    let resp_body: crate::models::Exhibit = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(resp_body.id.unwrap(), exhibit_id);
    assert_eq!(resp_body.name, "Art Exhibit");
    assert_eq!(resp_body.cluster, "Modern Art");
}

#[tokio::test]
async fn test_update_exhibit_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert a sample exhibit
    let db_conn = db.lock().await;
    let exhibit_repo = crate::db::repositories::ExhibitRepository::new(&*db_conn);
    let exhibit_id = exhibit_repo
        .create_exhibit(&crate::models::Exhibit {
            id: None,
            name: "Art Exhibit".to_string(),
            cluster: "Modern Art".to_string(),
            location: "Gallery 1".to_string(),
            status: "Active".to_string(),
            image_url: "http://localhost:3030/images/sample.jpg".to_string(),
            sponsor_name: Some("Art Sponsor".to_string()),
            sponsor_start_date: Some("2024-01-01".to_string()),
            sponsor_end_date: Some("2024-12-31".to_string()),
            part_ids: vec![],
            notes: vec![],
        })
        .unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = exhibit_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Define the updated exhibit payload
    let updated_exhibit = json!({
        "name": "Updated Art Exhibit",
        "cluster": "Contemporary Art",
        "location": "Gallery 2",
        "status": "Inactive",
        "image_url": "http://localhost:3030/images/updated_sample.jpg",
        "sponsor_name": "Updated Sponsor",
        "sponsor_start_date": "2025-01-01",
        "sponsor_end_date": "2025-12-31",
        "part_ids": [],
        "notes": []
    });

    // Perform the request
    let resp = warp::test::request()
        .method("PUT")
        .path(&format!("/exhibits/{}", exhibit_id))
        .json(&updated_exhibit)
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Retrieve the exhibit to verify updates
    let get_resp = warp::test::request()
        .method("GET")
        .path(&format!("/exhibits/{}", exhibit_id))
        .reply(&api)
        .await;

    let retrieved_exhibit: crate::models::Exhibit =
        serde_json::from_slice(get_resp.body()).unwrap();
    assert_eq!(retrieved_exhibit.name, "Updated Art Exhibit");
    assert_eq!(retrieved_exhibit.cluster, "Contemporary Art");
    assert_eq!(retrieved_exhibit.status, "Inactive");
}

#[tokio::test]
async fn test_delete_exhibit_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert a sample exhibit
    let db_conn = db.lock().await;
    let exhibit_repo = crate::db::repositories::ExhibitRepository::new(&*db_conn);
    let exhibit_id = exhibit_repo.create_exhibit(&get_test_exhibit()).unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = exhibit_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the delete request
    let resp = warp::test::request()
        .method("DELETE")
        .path(&format!("/exhibits/{}", exhibit_id))
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Attempt to retrieve the deleted exhibit
    let get_resp = warp::test::request()
        .method("GET")
        .path(&format!("/exhibits/{}", exhibit_id))
        .reply(&api)
        .await;

    // Assert that the exhibit is not found
    assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_exhibits_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert multiple exhibits
    let db_conn = db.lock().await;
    let exhibit_repo = crate::db::repositories::ExhibitRepository::new(&*db_conn);

    exhibit_repo
        .create_exhibit(&crate::models::Exhibit {
            id: None,
            name: "Art Exhibit 1".to_string(),
            cluster: "Modern Art".to_string(),
            location: "Gallery 1".to_string(),
            status: "Active".to_string(),
            image_url: "http://localhost:3030/images/sample1.jpg".to_string(),
            sponsor_name: Some("Art Sponsor 1".to_string()),
            sponsor_start_date: Some("2024-01-01".to_string()),
            sponsor_end_date: Some("2024-12-31".to_string()),
            part_ids: vec![],
            notes: vec![],
        })
        .unwrap();

    exhibit_repo
        .create_exhibit(&crate::models::Exhibit {
            id: None,
            name: "Art Exhibit 2".to_string(),
            cluster: "Contemporary Art".to_string(),
            location: "Gallery 2".to_string(),
            status: "Inactive".to_string(),
            image_url: "http://localhost:3030/images/sample2.jpg".to_string(),
            sponsor_name: Some("Art Sponsor 2".to_string()),
            sponsor_start_date: Some("2025-01-01".to_string()),
            sponsor_end_date: Some("2025-12-31".to_string()),
            part_ids: vec![],
            notes: vec![],
        })
        .unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = exhibit_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the list request
    let resp = warp::test::request()
        .method("GET")
        .path("/exhibits")
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert that the response body contains a list of exhibits
    let exhibits: Vec<crate::models::Exhibit> = serde_json::from_slice(resp.body()).unwrap();
    assert_eq!(exhibits.len(), 2);
}

#[tokio::test]
async fn test_get_random_exhibit_success() {
    // Initialize in-memory database
    let db = Arc::new(Mutex::new(DbConnection::new_in_memory().unwrap()));
    db.lock().await.setup_tables().unwrap();

    // Insert multiple exhibits
    let db_conn = db.lock().await;
    let exhibit_repo = crate::db::repositories::ExhibitRepository::new(&*db_conn);

    exhibit_repo
        .create_exhibit(&crate::models::Exhibit {
            id: None,
            name: "Art Exhibit 1".to_string(),
            cluster: "Modern Art".to_string(),
            location: "Gallery 1".to_string(),
            status: "Active".to_string(),
            image_url: "http://localhost:3030/images/sample1.jpg".to_string(),
            sponsor_name: Some("Art Sponsor 1".to_string()),
            sponsor_start_date: Some("2024-01-01".to_string()),
            sponsor_end_date: Some("2024-12-31".to_string()),
            part_ids: vec![],
            notes: vec![],
        })
        .unwrap();

    exhibit_repo
        .create_exhibit(&crate::models::Exhibit {
            id: None,
            name: "Art Exhibit 2".to_string(),
            cluster: "Contemporary Art".to_string(),
            location: "Gallery 2".to_string(),
            status: "Inactive".to_string(),
            image_url: "http://localhost:3030/images/sample2.jpg".to_string(),
            sponsor_name: Some("Art Sponsor 2".to_string()),
            sponsor_start_date: Some("2025-01-01".to_string()),
            sponsor_end_date: Some("2025-12-31".to_string()),
            part_ids: vec![],
            notes: vec![],
        })
        .unwrap();

    drop(db_conn); // Release the lock

    // Initialize the routes
    let api = exhibit_routes(db.clone()).recover(crate::errors::handle_rejection);

    // Perform the random exhibit request
    let resp = warp::test::request()
        .method("GET")
        .path("/exhibits/random")
        .reply(&api)
        .await;

    // Assert the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert that the response body contains one exhibit
    let exhibit: crate::models::Exhibit = serde_json::from_slice(resp.body()).unwrap();
    assert!(exhibit.name == "Art Exhibit 1" || exhibit.name == "Art Exhibit 2");
}
