// src/main.rs

#[macro_use]
extern crate rocket;

mod api;
mod db;
mod errors;
mod models;

use db::DbConnection;
use dotenv::dotenv;
use log::info;
use rocket::fairing::AdHoc;
use rocket::tokio::sync::Mutex;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use std::path::Path;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting Rocket server...");

    // Check if images directory exists, if not create it
    if !Path::new("images").exists() {
        std::fs::create_dir("images").expect("Failed to create images directory");
    }

    // Initialize the database connection without wrapping in Arc
    let db =
        Mutex::new(DbConnection::new("exhibits.db").expect("Failed to create database connection"));

    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| std::str::FromStr::from_str(s).unwrap())
        .collect();

    // Configure CORS
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&["http://localhost:1420"]))
        .allowed_methods(allowed_methods)
        .allowed_headers(AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
            "Origin",
            "X-Requested-With",
            "Access-Control-Allow-Origin",
        ]))
        .allow_credentials(false)
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build()
        .manage(db)
        .attach(cors) // Attach the CORS fairing
        .attach(AdHoc::on_ignite("Database Setup", |rocket| async {
            // Introduce a separate scope to ensure MutexGuard is dropped before returning rocket
            {
                let db = rocket
                    .state::<Mutex<DbConnection>>()
                    .expect("database state");
                let db_conn = db.lock().await;
                db_conn.setup_tables().expect("Failed to set up tables");
                // MutexGuard (`db_conn`) is dropped here as it goes out of scope
            }
            rocket
        }))
        .mount(
            "/",
            routes![
                api::github::report_bug_handler,
                api::exhibits::create_exhibit_handler,
                api::exhibits::get_exhibit_handler,
                api::exhibits::update_exhibit_handler,
                api::exhibits::delete_exhibit_handler,
                api::exhibits::list_exhibits_handler,
                api::exhibits::handle_random_exhibit,
                api::parts::create_part_handler,
                api::parts::get_part_handler,
                api::parts::update_part_handler,
                api::parts::delete_part_handler,
                api::parts::list_parts_handler,
                api::parts::get_parts_by_ids_handler,
                api::dev::handle_reset_db,
                api::dev::create_dummy_exhibits_handler
            ],
        )
        .mount("/images", rocket::fs::FileServer::from("images"))
        .register(
            "/",
            catchers![
                errors::not_found,
                errors::handle_invalid_request_body,
                errors::handle_method_not_allowed,
                errors::internal_server_error
            ],
        )
}
