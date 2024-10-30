// src/routes/exhibit_routes.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use crate::api::filters::with_db;
use crate::api::handlers::{
    create_exhibit_handler, delete_exhibit_handler, get_exhibit_handler, handle_random_exhibit,
    list_exhibits_handler, update_exhibit_handler,
};
use crate::db::DbConnection;

pub fn exhibit_routes(
    db: Arc<Mutex<DbConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create Exhibit: POST /exhibits
    let create_exhibit = warp::post()
        .and(warp::path("exhibits"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_exhibit_handler);

    // Get Exhibit by ID: GET /exhibits/:id
    let get_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(get_exhibit_handler);

    // Update Exhibit: PUT /exhibits/:id
    let update_exhibit = warp::put()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_exhibit_handler);

    // Delete Exhibit: DELETE /exhibits/:id
    let delete_exhibit = warp::delete()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(delete_exhibit_handler);

    // List All Exhibits: GET /exhibits
    let list_exhibits = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(list_exhibits_handler);

    // Get Random Exhibit: GET /exhibits/random
    let random_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path("random"))
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(handle_random_exhibit);

    // Combine all exhibit routes
    create_exhibit
        .or(get_exhibit)
        .or(update_exhibit)
        .or(delete_exhibit)
        .or(list_exhibits)
        .or(random_exhibit)
}
