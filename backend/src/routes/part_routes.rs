// src/routes/part_routes.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use crate::db::DbConnection;
use crate::handlers::part_handlers::*;
// Removed unused imports
// use crate::models::Part;
// use crate::repositories::PartRepository;
use crate::filters::db_filter::with_db; // Import the helper function

pub fn part_routes(
    db: Arc<Mutex<DbConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create Part: POST /parts
    let create_part = warp::post()
        .and(warp::path("parts"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_part_handler);

    // Get Part by ID: GET /parts/:id
    let get_part = warp::get()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(get_part_handler);

    // Update Part: PUT /parts/:id
    let update_part = warp::put()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_part_handler);

    // Delete Part: DELETE /parts/:id
    let delete_part = warp::delete()
        .and(warp::path("parts"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(delete_part_handler);

    // List All Parts: GET /parts
    let list_parts = warp::get()
        .and(warp::path("parts"))
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(list_parts_handler);

    // Get Parts with list of IDs: POST /parts/batch
    let get_parts_by_ids = warp::post()
        .and(warp::path("parts"))
        .and(warp::path("batch"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(get_parts_by_ids_handler);

    // Combine all part routes
    create_part
        .or(get_part)
        .or(update_part)
        .or(delete_part)
        .or(list_parts)
        .or(get_parts_by_ids)
}
