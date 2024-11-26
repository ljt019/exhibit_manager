// src/routes/part_routes.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use crate::api::filters::with_db;
use crate::api::handlers::{
    create_part_handler, delete_part_handler, get_part_handler, get_parts_by_ids_handler,
    list_parts_handler, update_part_handler,
};
use crate::db::DbConnection;

pub fn part_routes(
    db: Arc<Mutex<DbConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Common path for parts by ID
    let parts_by_id = warp::path!("parts" / i64).and(warp::path::end());

    // GET /parts/:id
    let get_part = warp::get()
        .and(parts_by_id.clone())
        .and(with_db(db.clone()))
        .and_then(get_part_handler);

    // PUT /parts/:id
    let update_part = warp::put()
        .and(parts_by_id.clone())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_part_handler);

    // DELETE /parts/:id
    let delete_part = warp::delete()
        .and(parts_by_id.clone())
        .and(with_db(db.clone()))
        .and_then(delete_part_handler);

    // Combine routes with the same path
    let parts_id_routes = get_part.or(update_part).or(delete_part);

    // Other routes
    // POST /parts/batch
    let get_parts_by_ids = warp::post()
        .and(warp::path!("parts" / "batch"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(get_parts_by_ids_handler);

    // POST /parts
    let create_part = warp::post()
        .and(warp::path!("parts"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_part_handler);

    // GET /parts
    let list_parts = warp::get()
        .and(warp::path!("parts"))
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(list_parts_handler);

    // Combine all routes in the correct order
    get_parts_by_ids
        .or(create_part)
        .or(list_parts)
        .or(parts_id_routes)
}
