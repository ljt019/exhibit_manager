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

// src/routes/exhibit_routes.rs
pub fn exhibit_routes(
    db: Arc<Mutex<DbConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Order routes from most specific to least specific

    // GET /exhibits/random - Must come before /:id to avoid conflict
    let random_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path("random"))
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(handle_random_exhibit);

    // GET /exhibits/:id
    let get_exhibit = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(get_exhibit_handler);

    // PUT /exhibits/:id
    let update_exhibit = warp::put()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_exhibit_handler);

    // DELETE /exhibits/:id
    let delete_exhibit = warp::delete()
        .and(warp::path("exhibits"))
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(delete_exhibit_handler);

    // GET /exhibits (list all)
    let list_exhibits = warp::get()
        .and(warp::path("exhibits"))
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(list_exhibits_handler);

    // POST /exhibits
    let create_exhibit = warp::post()
        .and(warp::path("exhibits"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_exhibit_handler);

    // Combine routes in order from most specific to least specific
    random_exhibit
        .or(get_exhibit)
        .or(update_exhibit)
        .or(delete_exhibit)
        .or(list_exhibits)
        .or(create_exhibit)
}
