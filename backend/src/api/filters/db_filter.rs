// src/filters/db_filter.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

use crate::db::DbConnection;

/// Helper function to pass the database connection to handlers
pub fn with_db(
    db: Arc<Mutex<DbConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<DbConnection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
