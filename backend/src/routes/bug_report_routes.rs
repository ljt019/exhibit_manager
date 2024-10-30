// src/routes/bug_report_routes.rs

use warp::Filter;

use crate::handlers::bug_report_handlers::*;

pub fn bug_report_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Report Bug: POST /report-bug
    let report_bug = warp::post()
        .and(warp::path("report-bug"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(report_bug_handler);

    report_bug
}
